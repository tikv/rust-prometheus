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
    use std::sync::RwLock;
    use std::sync::atomic::{Ordering, AtomicPtr, AtomicU64};

    use super::Entry;

    pub struct MapVec<V: Clone> {
        entries: [Entry<AtomicU64, AtomicPtr<V>>; 8],
        heap: RwLock<[Option<V>; 8]>,
    }

    impl<V: Clone> MapVec<V> {
        pub fn new() -> MapVec<V> {
            MapVec {
                entries: [Entry::<AtomicU64, AtomicPtr<V>>::default(),
                          Entry::default(),
                          Entry::default(),
                          Entry::default(),
                          Entry::default(),
                          Entry::default(),
                          Entry::default(),
                          Entry::default()],
                heap: RwLock::new([None, None, None, None, None, None, None, None]),
            }
        }

        pub fn get(&self, k: &u64) -> Option<V> {
            self.entries
                .iter()
                .find(|&v| *k == v.key().load(Ordering::Relaxed))
                .and_then(|entry| {
                    let ptr = entry.value().load(Ordering::Relaxed);
                    if 0 != ptr as usize {
                        unsafe {
                            return Some((*ptr).clone());
                        }
                    } else {
                        return None;
                    }
                })
        }

        pub fn get_or_insert(&self, k: u64, v: V) -> Option<V> {
            self.get(&k).or_else(|| {
                // insert.
                self.entries
                    .iter()
                    .enumerate()
                    .find(|&(idx, entry)| {
                        // try find an empty slot.
                        let found = 0 == entry.key().compare_and_swap(0, k, Ordering::Relaxed);
                        if found {
                            // key has been updated, updating value and heap.
                            let mut heap = self.heap.write().unwrap();
                            heap[idx] = Some(v.clone());
                            let ptr = heap[idx].as_mut().unwrap() as *mut V;
                            entry.value().store(ptr, Ordering::Relaxed);
                        }
                        found
                    })
                    .map(|(_, entry)| {
                        let ptr = entry.value().load(Ordering::Relaxed);
                        unsafe { (*ptr).clone() }
                    })
            })
        }

        pub fn remove(&self, k: &u64) -> Option<V> {
            self.entries
                .iter()
                .position(|entry| *k == entry.key().load(Ordering::Relaxed))
                .and_then(|idx| {
                    let key = self.entries[idx].key().swap(0, Ordering::Relaxed);
                    if key == 0 {
                        return None;
                    } else {
                        let ptr = self.entries[idx]
                            .value()
                            .swap((0 as *mut V),
                                  Ordering::Relaxed) as usize;
                        if ptr == 0 {
                            return None;
                        }
                        // update value and heap.
                        self.entries[idx].value().store((0 as *mut V), Ordering::Relaxed);
                        let mut heap = self.heap.write().unwrap();
                        return heap[idx].take();
                    }
                })
        }

        pub fn clear(&self) {
            let mut heap = self.heap.write().unwrap();
            for _ in self.entries
                .iter()
                .enumerate()
                .inspect(|&(idx, entry)| {
                    entry.key().store(0, Ordering::Relaxed);
                    let ptr = entry.value().swap((0 as *mut V), Ordering::Relaxed) as usize;
                    if ptr != 0 {
                        heap[idx].take();
                    }
                }) {}
        }

        pub fn get_vec(&self) -> Vec<Entry<u64, V>> {
            self.entries
                .iter()
                .fold(Vec::new(), |mut vec, ref entry| {
                    let key = entry.key().load(Ordering::Relaxed);
                    let ptr = entry.value().load(Ordering::Relaxed);
                    if 0 != ptr as usize {
                        let v = unsafe { (*ptr).clone() };
                        vec.push(Entry::new(key, v));
                    }
                    vec
                })
        }

        pub fn len(&self) -> usize {
            self.entries
                .iter()
                .fold(0, |acc, ref v| {
                    if 0 != v.value().load(Ordering::Relaxed) as usize {
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
