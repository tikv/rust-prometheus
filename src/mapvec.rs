// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Default)]
pub struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K, V> Entry<K, V> {
    #[inline]
    pub fn new(key: K, value: V) -> Entry<K, V> {
        Entry {
            key: key,
            value: value,
        }
    }

    #[inline]
    pub fn key(&self) -> &K {
        &self.key
    }

    #[inline]
    pub fn value(&self) -> &V {
        &self.value
    }

    #[cfg(not(feature = "nightly"))]
    #[inline]
    pub fn take_value(self) -> V {
        self.value
    }
}

#[cfg(not(feature = "nightly"))]
pub use self::rwlock::MapVec;

#[cfg(not(feature = "nightly"))]
mod rwlock {
    use std::sync::{RwLock, RwLockReadGuard};

    use super::Entry;

    pub struct MapVec<K: Eq, V: Clone> {
        entries: RwLock<Vec<Entry<K, V>>>,
    }

    impl<K: Eq, V: Clone> MapVec<K, V> {
        pub fn new() -> MapVec<K, V> {
            MapVec { entries: RwLock::new(Vec::new()) }
        }

        pub fn get(&self, k: &K) -> Option<V> {
            self.entries
                .read()
                .unwrap()
                .iter()
                .find(|&v| k == v.key())
                .map(|entry| entry.value().clone())
        }

        pub fn get_or_insert(&self, k: K, v: V) -> Option<V> {
            let mut entries = self.entries.write().unwrap();
            entries.iter()
                .position(|v| &k == v.key())
                .map(|idx| entries[idx].value().clone())
                .or_else(|| {
                    entries.push(Entry::new(k, v.clone()));
                    Some(v)
                })
        }

        pub fn remove(&self, k: &K) -> Option<V> {
            let mut entries = self.entries.write().unwrap();
            entries.iter()
                .position(|v| k == v.key())
                .map(|idx| entries.remove(idx).take_value())
        }

        pub fn clear(&self) {
            self.entries
                .write()
                .unwrap()
                .clear();
        }

        pub fn get_vec(&self) -> RwLockReadGuard<Vec<Entry<K, V>>> {
            self.entries.read().unwrap()
        }

        pub fn len(&self) -> usize {
            self.entries
                .read()
                .unwrap()
                .len()
        }
    }
}

#[cfg(feature = "nightly")]
pub use self::atomic::MapVec;

#[cfg(feature = "nightly")]
mod atomic {
    use std::usize::MAX;
    use std::sync::atomic::{Ordering, AtomicPtr, AtomicU64};

    use super::Entry;

    const LOCKED: usize = MAX;
    const EMPTY_VALUE: usize = 0;
    const EMPTY_KEY: u64 = 0;

    /// Requirements:
    ///   - V has to be wrapped by `Arc`, or internally be wrapped by `Arc`.
    ///   - key can not be 0 or `0xFFFFFFFFFFFFFFFF`.`
    pub struct MapVec<V: Clone> {
        // Guarantee: `AtomicPtr`s always point to a valid address,
        //            if it is not `0` or `MAX`.
        entries: [Entry<AtomicU64, AtomicPtr<V>>; 8],
    }

    impl<V: Clone> MapVec<V> {
        pub fn new() -> MapVec<V> {
            MapVec {
                // TODO: new array macro
                entries: [Entry::<AtomicU64, AtomicPtr<V>>::default(),
                          Entry::default(),
                          Entry::default(),
                          Entry::default(),
                          Entry::default(),
                          Entry::default(),
                          Entry::default(),
                          Entry::default()],
            }
        }

        pub fn get(&self, k: &u64) -> Option<V> {
            self.entries
                .iter()
                .find(|&v| *k == v.key().load(Ordering::Relaxed))
                .and_then(|entry| {
                    // TODO: what if this entry has been removed?
                    loop {
                        let current = entry.value().load(Ordering::Relaxed);
                        if 0 == current as usize {
                            // fail fast.
                            return None;
                        }

                        // wait lock
                        if LOCKED == current as usize {
                            // TODO: thread yield?
                            continue;
                        }
                        // current is a pointer that points to a valid address.

                        // try lock.
                        let swapped = entry.value()
                            .compare_and_swap(current, LOCKED as *mut V, Ordering::Relaxed);
                        if swapped != current {
                            continue;
                        }
                        // we got lock!

                        // swapped is guaranteed to be valid.
                        let v = unsafe { (*swapped).clone() };

                        // try restore the pervious value, while releasing the
                        // lock, the guarantee of value is still hold.
                        let lock = entry.value()
                            .compare_and_swap(LOCKED as *mut V, swapped, Ordering::Relaxed);
                        debug_assert_eq!(lock as usize, LOCKED);

                        return Some(v);
                    }
                })
        }

        pub fn get_or_insert(&self, k: u64, v: V) -> Option<V> {
            self.get(&k).or_else(|| {
                // put it to heap first.
                let mut ptr = Some(Box::into_raw(Box::new(v.clone())));

                // try find an empty slot.
                for entry in self.entries.iter() {
                    let key = entry.key().compare_and_swap(0, k, Ordering::Relaxed);
                    if key != 0 {
                        continue;
                    }
                    // the key has been updated, the value will be updated.

                    loop {
                        // wait lock.
                        let current = entry.value().load(Ordering::Relaxed);
                        if LOCKED == current as usize {
                            // TODO: thread yield?
                            continue;
                        }
                        // current is EMPTY_VALUE or a pointer that points to
                        // a valid address.

                        // try lock.
                        let swapped = entry.value()
                            .compare_and_swap(current, LOCKED as *mut V, Ordering::Relaxed);
                        if current != swapped {
                            continue;
                        }
                        // we got lock!

                        // do not forget to drop the pervious value!
                        if EMPTY_VALUE != swapped as usize {
                            unsafe {
                                drop(Box::from_raw(swapped));
                            }
                        }

                        // prepare return value.
                        debug_assert!(ptr.is_some());
                        let new_ptr = ptr.take().unwrap();
                        let ret = unsafe { (*new_ptr).clone() };

                        // try update the value, while releasing the lock, the
                        // guarantee of value is still hold.
                        let lock = entry.value()
                            .compare_and_swap(LOCKED as *mut V, new_ptr, Ordering::Relaxed);
                        debug_assert_eq!(lock as usize, LOCKED);

                        return Some(ret);
                    }
                }

                // out of space!
                debug_assert!(ptr.is_some());
                ptr.and_then(|ptr| {
                    unsafe {
                        drop(Box::from_raw(ptr));
                    }
                    None
                })
            })
        }

        pub fn remove(&self, k: &u64) -> Option<V> {
            self.entries
                .iter()
                .position(|entry| {
                    *k == entry.key().compare_and_swap(*k, EMPTY_KEY, Ordering::Relaxed)
                })
                .and_then(|idx| {
                    // try lock
                    loop {
                        let ref entry = self.entries[idx];

                        // wait lock.
                        let current = entry.value().load(Ordering::Relaxed);
                        if LOCKED == current as usize {
                            // TODO: thread yield?
                            continue;
                        }
                        // current is EMPTY_VALUE or a pointer that points to
                        // a valid address.

                        // try lock.
                        let swapped = entry.value()
                            .compare_and_swap(current, LOCKED as *mut V, Ordering::Relaxed);
                        if current != swapped {
                            continue;
                        }
                        // we got lock!

                        let ret = if EMPTY_VALUE != swapped as usize {
                            // clone first, then drop the value.
                            let ret = unsafe {
                                let ret = (*swapped).clone();
                                drop(Box::from_raw(swapped));
                                ret
                            };

                            Some(ret)
                        } else {
                            // TODO: bug?
                            None
                        };

                        // release lock, and replace LOCKED with EMPTY_VALUE.
                        let lock = entry.value()
                            .compare_and_swap(LOCKED as *mut V,
                                              EMPTY_VALUE as *mut V,
                                              Ordering::Relaxed);
                        debug_assert_eq!(lock as usize, LOCKED);

                        return ret;
                    }
                })
        }

        pub fn clear(&self) {
            for entry in self.entries.iter() {
                entry.key().store(EMPTY_KEY, Ordering::Relaxed);
                // it is ok to just remove keys, the underlying data will be
                // dropped during the updating of this entry.
            }
        }

        pub fn get_vec(&self) -> Vec<Entry<u64, V>> {
            let mut vec = Vec::new();

            for entry in self.entries.iter() {
                let key = entry.key().load(Ordering::Relaxed);
                if EMPTY_KEY != key {
                    // wait lock.
                    let current = entry.value().load(Ordering::Relaxed);
                    if LOCKED == current as usize {
                        // TODO: thread yield?
                        continue;
                    }
                    // current is EMPTY_VALUE or a pointer that points to
                    // a valid address.

                    // try lock.
                    let swapped = entry.value()
                        .compare_and_swap(current, LOCKED as *mut V, Ordering::Relaxed);
                    if current != swapped {
                        continue;
                    }
                    // we got lock!

                    if EMPTY_VALUE != swapped as usize {
                        let v = unsafe { (*swapped).clone() };
                        vec.push(Entry::new(key, v));
                    }

                    // release lock, and replace it with the pervious value.
                    let lock = entry.value()
                        .compare_and_swap(LOCKED as *mut V, swapped, Ordering::Relaxed);
                    debug_assert_eq!(lock as usize, LOCKED);
                }
            }

            vec
        }

        pub fn len(&self) -> usize {
            self.entries
                .iter()
                .fold(0, |acc, ref v| {
                    if 0 != v.key().load(Ordering::Relaxed) as usize {
                        return acc + 1;
                    }
                    acc
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapvec() {
        let mv = MapVec::new();

        assert!(mv.get(&0xdeadbeef_u64).is_none());

        mv.get_or_insert(0xdeadbeef_u64, "0xdeadbeef_u64".to_owned());
        assert!(mv.get(&0xdeadbeef_u64).is_some());
        assert_eq!(mv.get(&0xdeadbeef_u64).unwrap(),
                   "0xdeadbeef_u64".to_owned());

        mv.get_or_insert(0xdeadbeef_u64, "0xbeefdead".to_owned());
        assert!(mv.get(&0xdeadbeef_u64).is_some());
        assert_eq!(mv.get(&0xdeadbeef_u64).unwrap(),
                   "0xdeadbeef_u64".to_owned());
        assert_eq!(mv.get_vec().len(), 1);

        mv.get_or_insert(0xfabaceae_u64, "0xfabaceae_u64".to_owned());
        assert!(mv.get(&0xfabaceae_u64).is_some());
        assert_eq!(mv.get(&0xfabaceae_u64).unwrap(),
                   "0xfabaceae_u64".to_owned());

        assert!(mv.get(&0xdeadbeef_u64).is_some());
        assert_eq!(mv.get(&0xdeadbeef_u64).unwrap(),
                   "0xdeadbeef_u64".to_owned());
        assert_eq!(mv.get_vec().len(), 2);

        let deadbeef = mv.remove(&0xdeadbeef_u64);
        assert!(deadbeef.is_some());
        assert_eq!(deadbeef.unwrap(), "0xdeadbeef_u64".to_owned());
        assert_eq!(mv.get_vec().len(), 1);

        mv.clear();
        assert!(mv.remove(&0xdeadbeef_u64).is_none());
        assert!(mv.remove(&0xfabaceae_u64).is_none());
        assert_eq!(mv.get_vec().len(), 0);
    }
}
