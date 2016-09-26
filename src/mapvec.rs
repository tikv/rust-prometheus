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

#[cfg(not(feature = "nightly"))]
pub use self::rwlock::MapVec;

#[cfg(feature = "nightly")]
pub use self::atomic::{MapVec, set_capacity};

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
mod rwlock {
    use std::sync::{RwLock, RwLockReadGuard};

    use super::Entry;

    pub struct MapVec<V: Clone> {
        entries: RwLock<Vec<Entry<u64, V>>>,
    }

    impl<V: Clone> MapVec<V> {
        /// Creates an empty `MapVec`.
        pub fn new() -> MapVec<V> {
            MapVec { entries: RwLock::new(Vec::new()) }
        }

        /// Returns a cloned value corresponding to the key.
        pub fn get(&self, k: &u64) -> Option<V> {
            self.entries
                .read()
                .unwrap()
                .iter()
                .find(|&v| k == v.key())
                .map(|entry| entry.value().clone())
        }

        /// Returns a cloned value corresponding to the key.
        /// Inserts a key-value pair into the map, if the map did not have this
        /// key present.
        /// If the map did have this key present, the value will not be updated.
        pub fn get_or_insert(&self, k: u64, v: V) -> Option<V> {
            let mut entries = self.entries.write().unwrap();
            entries.iter()
                .position(|v| &k == v.key())
                .map(|idx| entries[idx].value().clone())
                .or_else(|| {
                    entries.push(Entry::new(k, v.clone()));
                    Some(v)
                })
        }

        /// Removes a key from the map, returning the value at the key if the
        /// key was previously in the map.
        pub fn remove(&self, k: &u64) -> Option<V> {
            let mut entries = self.entries.write().unwrap();
            entries.iter()
                .position(|v| k == v.key())
                .map(|idx| entries.remove(idx).take_value())
        }

        /// Clears the map, removing all key-value pairs. Keeps the allocated
        /// memory for reuse.
        pub fn clear(&self) {
            self.entries
                .write()
                .unwrap()
                .clear();
        }

        /// Returns the internal `Vec`.
        pub fn get_vec(&self) -> RwLockReadGuard<Vec<Entry<u64, V>>> {
            self.entries.read().unwrap()
        }

        /// Returns the number of elements in the map.
        pub fn len(&self) -> usize {
            self.entries
                .read()
                .unwrap()
                .len()
        }
    }
}

#[cfg(feature = "nightly")]
mod atomic {
    use std::usize::MAX;
    use std::sync::atomic::{Ordering, AtomicPtr, AtomicU64, AtomicUsize, ATOMIC_USIZE_INIT};

    use super::Entry;

    const LOCKED: usize = MAX;
    const EMPTY_VALUE: usize = 0;
    const EMPTY_KEY: u64 = 0;

    pub const DEFAULT_CAP: usize = 16;
    static CAP: AtomicUsize = ATOMIC_USIZE_INIT;

    /// Set the global capacity of all `MepVec`.
    pub fn set_capacity(cap: usize) {
        CAP.store(cap, Ordering::Relaxed);
    }

    /// An atomic map with a limited capacity. It does not provide strong
    /// consistency.
    ///
    /// Requirements:
    ///   - V has to be wrapped by `Arc`, or internally be wrapped by `Arc`.
    ///   - key can not be 0 or `0xFFFFFFFFFFFFFFFF`.`
    pub struct MapVec<V: Clone> {
        // Guarantee: `AtomicPtr`s always point to a valid address,
        //            if it is not `0` or `MAX`.
        entries: Vec<Entry<AtomicU64, AtomicPtr<V>>>,
    }

    impl<V: Clone> MapVec<V> {
        /// Creates an empty `MapVec`.
        pub fn new() -> MapVec<V> {
            let mut cap = CAP.load(Ordering::Relaxed);
            if cap == 0 {
                cap = DEFAULT_CAP;
            }

            let mut vec = Vec::with_capacity(cap);
            for _ in 0..cap {
                vec.push(Entry::default());
            }

            MapVec { entries: vec }
        }

        /// Returns a cloned value corresponding to the key.
        pub fn get(&self, k: &u64) -> Option<V> {
            self.entries
                .iter()
                .find(|&v| *k == v.key().load(Ordering::Relaxed))
                .and_then(|entry| {
                    loop {
                        let current = entry.value().load(Ordering::Relaxed);
                        if EMPTY_VALUE == current as usize {
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
                        // lock, the guarantee is still hold.
                        let lock = entry.value()
                            .compare_and_swap(LOCKED as *mut V, swapped, Ordering::Relaxed);
                        debug_assert_eq!(lock as usize, LOCKED);

                        return Some(v);
                    }
                })
        }

        /// Returns a cloned value corresponding to the key.
        /// Inserts a key-value pair into the map, if the map did not have this
        /// key present.
        /// If the map did have this key present, the value will not be updated.
        pub fn get_or_insert(&self, k: u64, v: V) -> Option<V> {
            self.get(&k).or_else(|| {
                // put it to heap first.
                let mut ptr = Some(Box::into_raw(Box::new(v.clone())));

                // try to find an empty slot.
                for entry in &self.entries {
                    let key = entry.key().compare_and_swap(EMPTY_KEY, k, Ordering::Relaxed);
                    if key != EMPTY_KEY {
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

        /// Removes a key from the map, returning the value at the key if the
        /// key was previously in the map.
        pub fn remove(&self, k: &u64) -> Option<V> {
            self.entries
                .iter()
                .position(|entry| {
                    *k == entry.key().compare_and_swap(*k, EMPTY_KEY, Ordering::Relaxed)
                })
                .and_then(|idx| {
                    // try lock
                    loop {
                        let entry = &self.entries[idx];

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

        /// Clears the map, removing all key-value pairs. Keeps the allocated
        /// memory for reuse.
        pub fn clear(&self) {
            for entry in &self.entries {
                entry.key().store(EMPTY_KEY, Ordering::Relaxed);
                // it is ok to just remove keys, the underlying data will be
                // dropped during the updating of this entry.
            }
        }

        /// Returns a cloned internal `Vec`.
        pub fn get_vec(&self) -> Vec<Entry<u64, V>> {
            let mut vec = Vec::new();

            for entry in &self.entries {
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

        /// Returns the number of elements in the map.
        pub fn len(&self) -> usize {
            self.entries
                .iter()
                .fold(0, |acc, v| {
                    if EMPTY_KEY != v.key().load(Ordering::Relaxed) {
                        return acc + 1;
                    }
                    acc
                })
        }
    }

    #[cfg(test)]
    mod tests {
        use std::sync::Arc;

        use super::*;

        #[test]
        fn test_capacity() {
            let m = MapVec::new();
            let mut key = 1;
            loop {
                if let None = m.get_or_insert(key, Arc::new(key)) {
                    break;
                }
                key += 1;
            }
            assert_eq!(m.len(), DEFAULT_CAP);

            set_capacity(DEFAULT_CAP * 2);
            let m1 = MapVec::new();
            loop {
                if let None = m1.get_or_insert(key, Arc::new(key)) {
                    break;
                }
                key += 1;
            }
            assert_eq!(m1.len(), DEFAULT_CAP * 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::sync::Arc;
    use std::sync::atomic::{Ordering, AtomicUsize};

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

    #[test]
    fn test_mapvec_concurrent() {
        lazy_static! {
            static ref MV: Arc<MapVec<Arc<AtomicUsize>>> = Arc::new(MapVec::new());
        }
        let counter = Arc::new(AtomicUsize::new(1));
        let loops = 1024 * 1024;

        let c1 = counter.clone();
        let m1 = MV.as_ref();
        let t1 = thread::spawn(move || {
            for i in 0..loops {
                m1.get_or_insert(1, c1.clone()).unwrap().fetch_add(1, Ordering::Relaxed);
                m1.get(&1).map(|c| c.fetch_add(1, Ordering::Relaxed));
                m1.remove(&1).map(|c| c.fetch_add(1, Ordering::Relaxed));
                if 0 == i % 100 {
                    thread::yield_now();
                }
            }
        });

        let c2 = counter.clone();
        let m2 = MV.as_ref();
        let t2 = thread::spawn(move || {
            for i in 0..loops {
                m2.get_or_insert(2, c2.clone()).unwrap().fetch_add(1, Ordering::Relaxed);
                m2.get(&2).map(|c| c.fetch_add(1, Ordering::Relaxed));
                m2.remove(&2).map(|c| c.fetch_add(1, Ordering::Relaxed));
                if 0 == i % 100 {
                    thread::yield_now();
                }
            }
        });

        let c3 = counter.clone();
        let m3 = MV.as_ref();
        let t3 = thread::spawn(move || {
            for i in 0..loops {
                m3.get_or_insert(3, c3.clone()).unwrap().fetch_add(1, Ordering::Relaxed);
                m3.get(&3).map(|c| c.fetch_add(1, Ordering::Relaxed));
                m3.remove(&3).map(|c| c.fetch_add(1, Ordering::Relaxed));
                if 0 == i % 100 {
                    thread::yield_now();
                }
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();

        assert_eq!(MV.len(), 0);
    }
}
