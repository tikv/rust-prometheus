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

        pub fn with_capacity(cap: usize) -> MapVec<V> {
            let vec = Vec::with_capacity(cap);
            MapVec { entries: RwLock::new(vec) }
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
    use std::sync::atomic::{Ordering, AtomicU64, AtomicUsize, ATOMIC_USIZE_INIT};

    use crossbeam::mem::epoch::{self, Atomic, Owned, Guard};

    use super::Entry;

    pub const KEY_EMPTY: u64 = ::std::u64::MIN;
    pub const KEY_TOMBSTONE: u64 = ::std::u64::MAX;

    pub fn validate_key(k: u64) -> bool {
        k != KEY_EMPTY && k != KEY_TOMBSTONE
    }

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
    ///   - key can not be `0` or `0xFFFFFFFFFFFFFFFF`.
    ///   - value can not be `0` or `0xFFFFFFFFFFFFFFFF`.
    pub struct MapVec<V: Clone> {
        // Guarantee: `AtomicPtr`s always point to a valid address,
        //            if it is not `0` or `MAX`.
        entries: Vec<Entry<AtomicU64, Atomic<V>>>,
    }

    impl<V: Clone> MapVec<V> {
        /// Creates an empty `MapVec`.
        pub fn new() -> MapVec<V> {
            let mut cap = CAP.load(Ordering::Relaxed);
            if cap == 0 {
                cap = DEFAULT_CAP;
            }

            MapVec::with_capacity(cap)
        }

        pub fn with_capacity(cap: usize) -> MapVec<V> {
            let mut vec = Vec::with_capacity(cap);
            for _ in 0..cap {
                let e = Entry::new(AtomicU64::new(KEY_EMPTY), Atomic::null());
                vec.push(e);
            }

            MapVec { entries: vec }
        }

        /// Returns a cloned value corresponding to the key.
        pub fn get(&self, k: &u64) -> Option<V> {
            if !validate_key(*k) {
                return None;
            }

            for e in &self.entries {
                loop {
                    let key = e.key().load(Ordering::Relaxed);
                    if key == *k {
                        let guard = epoch::pin();
                        if let Some(v) = e.value().load(Ordering::Relaxed, &guard) {
                            return Some((*v).clone());
                        } else {
                            // we are in the middle of `get_or_insert`, wait for it.
                            continue;
                        }
                    }

                    // not found.
                    if key == KEY_EMPTY {
                        return None;
                    }

                    // it is not the key we want and it is not an empty key,
                    // then it must be something else.
                    break;
                }
            }

            None
        }

        /// Returns a cloned value corresponding to the key.
        /// Inserts a key-value pair into the map, if the map did not have this
        /// key present.
        /// If the map did have this key present, the value will not be updated.
        ///
        /// It always return a Some(V) unless, there is no space to inert.
        pub fn get_or_insert(&self, k: u64, v: V) -> Option<V> {
            if !validate_key(k) {
                return None;
            }

            let mut owned = Owned::new(v);
            for e in &self.entries {
                loop {
                    let key = e.key().load(Ordering::Relaxed);

                    if key == k {
                        // get,
                        // found the target entry.

                        let guard = epoch::pin();
                        match e.value().load(Ordering::Relaxed, &guard) {
                            Some(v) => return Some((*v).clone()),
                            None => continue,
                        }
                    }

                    if key != KEY_EMPTY {
                        // it is not the key we want and it is not an empty key,
                        // then it must be something else.
                        break;
                    }

                    // insert,
                    // empty entry, try cas the key then the value.

                    // compare_exchange the key.
                    // here comes three possable situation.
                    //   succeed: prev == KEY_EMPTY
                    //   failed:
                    //     1. prev != k
                    //     2. prev == k
                    match e.key()
                        .compare_exchange(KEY_EMPTY, k, Ordering::Relaxed, Ordering::Relaxed) {
                        Err(prev) => {
                            // prev == k but compare_exchange failed.
                            //
                            // other thread win the race, continue to get.
                            if prev == k {
                                continue;
                            }
                            // prev != k
                            //
                            // other thread insert a new value, break to next entry.
                            break;
                        }

                        Ok(_) => {
                            // update the key successfully. now update the value.
                            let guard = epoch::pin();
                            match e.value().cas_and_ref(None, owned, Ordering::Relaxed, &guard) {
                                // cas succeed.
                                Ok(s) => return Some((*s).clone()),

                                // TODO: when will it happen?
                                Err(o) => owned = o,
                            }
                        }
                    }
                }
            }

            // TODO: running out space?
            None
        }

        #[inline]
        fn reclaim_value(v: &Atomic<V>, guard: Guard) -> Option<V> {
            let swapped = v.swap(None, Ordering::Relaxed, &guard).map(|shared| {
                unsafe {
                    guard.unlinked(shared);
                }
                (*shared).clone()
            });

            swapped
        }

        /// Removes a key from the map, returning the value at the key if the
        /// key was previously in the map.
        pub fn remove(&self, k: &u64) -> Option<V> {
            if !validate_key(*k) {
                return None;
            }

            for e in &self.entries {
                if let Ok(_) = e.key()
                    .compare_exchange(*k, KEY_TOMBSTONE, Ordering::Relaxed, Ordering::Relaxed) {

                    let guard = epoch::pin();
                    return Self::reclaim_value(e.value(), guard);
                }
            }

            None
        }

        /// Clears the map, removing all key-value pairs.
        pub fn clear(&self) {
            for e in &self.entries {
                let k = e.key().load(Ordering::Relaxed);
                if k == KEY_EMPTY {
                    return;
                }
                if k == KEY_TOMBSTONE {
                    continue;
                }

                if let Ok(_) = e.key()
                    .compare_exchange(k, KEY_TOMBSTONE, Ordering::Relaxed, Ordering::Relaxed) {

                    let guard = epoch::pin();
                    Self::reclaim_value(e.value(), guard);
                }
            }
        }

        /// Returns a cloned internal `Vec`.
        pub fn get_vec(&self) -> Vec<Entry<u64, V>> {
            let mut vec = Vec::new();

            for e in &self.entries {
                let k = e.key().load(Ordering::Relaxed);
                if k == KEY_EMPTY {
                    break;
                }
                if k == KEY_TOMBSTONE {
                    continue;
                }

                let guard = epoch::pin();
                e.value().load(Ordering::Relaxed, &guard).map(|v| {
                    vec.push(Entry::new(k, (*v).clone()));
                });
            }

            vec
        }

        /// Returns the number of elements in the map.
        pub fn len(&self) -> usize {
            self.entries
                .iter()
                .fold(0, |acc, v| {
                    let k = v.key().load(Ordering::Relaxed);
                    if validate_key(k) {
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
    fn test_mapvec_concurrent_get_or_insert() {
        let mapvec = Arc::new(MapVec::with_capacity(1024));
        let counter = Arc::new(AtomicUsize::new(0));

        let c1 = counter.clone();
        let m1 = mapvec.clone();
        let t1 = thread::Builder::new()
            .name("t1".to_owned())
            .spawn(move || {
                for i in 1..513 {
                    m1.get_or_insert(i, c1.clone()).unwrap().fetch_add(1, Ordering::Relaxed);

                    if 0 == i % 32 {
                        thread::yield_now();
                    }
                }
            })
            .unwrap();

        let c2 = counter.clone();
        let m2 = mapvec.clone();
        let t2 = thread::Builder::new()
            .name("t2".to_owned())
            .spawn(move || {
                for i in 1..513 {
                    m2.get_or_insert(i, c2.clone()).unwrap().fetch_add(1, Ordering::Relaxed);

                    if 0 == i % 20 {
                        thread::yield_now();
                    }
                }
            })
            .unwrap();

        t1.join().unwrap();
        t2.join().unwrap();

        assert_eq!(mapvec.len(), 512);
        assert_eq!(counter.load(Ordering::Relaxed), 1024);
    }

    #[test]
    fn test_mapvec_concurrent_insert_get() {
        let mapvec = Arc::new(MapVec::with_capacity(1024));
        let counter = Arc::new(AtomicUsize::new(0));

        let c1 = counter.clone();
        let m1 = mapvec.clone();
        let t1 = thread::Builder::new()
            .name("t1".to_owned())
            .spawn(move || {
                for i in 1..513 {
                    m1.get_or_insert(i, c1.clone()).unwrap();
                    m1.get(&i).unwrap().fetch_add(1, Ordering::Relaxed);

                    if 0 == i % 32 {
                        thread::yield_now();
                    }
                }
            })
            .unwrap();

        let c2 = counter.clone();
        let m2 = mapvec.clone();
        let t2 = thread::Builder::new()
            .name("t2".to_owned())
            .spawn(move || {
                for i in 1..513 {
                    m2.get_or_insert(i, c2.clone()).unwrap();
                    m2.get(&i).unwrap().fetch_add(1, Ordering::Relaxed);

                    if 0 == i % 20 {
                        thread::yield_now();
                    }
                }
            })
            .unwrap();

        t1.join().unwrap();
        t2.join().unwrap();

        assert_eq!(mapvec.len(), 512);
        assert_eq!(counter.load(Ordering::Relaxed), 1024);
    }

    #[test]
    fn test_mapvec_concurrent_insert_remove() {
        let mapvec = Arc::new(MapVec::with_capacity(1024));
        let counter = Arc::new(AtomicUsize::new(0));

        let c1 = counter.clone();
        let m1 = mapvec.clone();
        let t1 = thread::Builder::new()
            .name("t1".to_owned())
            .spawn(move || {
                for i in 1..513 {
                    m1.get_or_insert(i, c1.clone()).unwrap();
                    m1.remove(&i).map(|v| v.fetch_add(1, Ordering::Relaxed));

                    if 0 == i % 32 {
                        thread::yield_now();
                    }
                }
            })
            .unwrap();

        let c2 = counter.clone();
        let m2 = mapvec.clone();
        let t2 = thread::Builder::new()
            .name("t2".to_owned())
            .spawn(move || {
                for i in 1..513 {
                    m2.get_or_insert(i, c2.clone()).unwrap();
                    m2.remove(&i).map(|v| v.fetch_add(1, Ordering::Relaxed));

                    if 0 == i % 20 {
                        thread::yield_now();
                    }
                }
            })
            .unwrap();

        t1.join().unwrap();
        t2.join().unwrap();

        assert_eq!(mapvec.len(), 0);
    }
}
