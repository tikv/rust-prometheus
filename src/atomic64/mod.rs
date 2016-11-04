// Copyright 2014 The Prometheus Authors
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


cfg_if! {
    if #[cfg(feature = "nightly")] {
        // Prefer AtomicU64 if available.
        #[path = "atomic.rs"]
        mod imp;
    } else {
        // Fall back to RwLock based version.
        #[path = "rwlock.rs"]
        mod imp;
    }
}


pub struct AtomicF64 {
    inner: imp::F64,
}


impl AtomicF64 {
    pub fn new(val: f64) -> AtomicF64 {
        AtomicF64 {
            inner: imp::F64::new(val),
        }
    }

    #[inline]
    pub fn get(&self) -> f64 {
        self.inner.get()
    }

    #[inline]
    pub fn set(&self, val: f64) {
        self.inner.set(val)
    }

    #[inline]
    pub fn inc_by(&self, delta: f64) {
        self.inner.inc_by(delta)
    }
}


pub struct AtomicU64 {
    inner: imp::U64,
}

impl AtomicU64 {
    pub fn new(val: u64) -> AtomicU64 {
        AtomicU64 {
            inner: imp::U64::new(val),
        }
    }

    #[inline]
    pub fn get(&self) -> u64 {
        self.inner.get()
    }

    #[inline]
    pub fn inc_by(&self, delta: u64) {
        self.inner.inc_by(delta)
    }
}


#[cfg(test)]
mod test {
    use std::f64::consts::PI;
    use std::f64::{self, EPSILON};

    use super::*;

    #[test]
    fn test_atomicf64() {
        let table: Vec<f64> = vec![0.0, 1.0, PI, f64::MIN, f64::MAX];

        for f in table {
            assert!((f - AtomicF64::new(f).get()).abs() < EPSILON);
        }
    }

    #[test]
    fn test_atomicu64() {
        let au64 = AtomicU64::new(0);
        assert_eq!(au64.get(), 0);

        au64.inc_by(1);
        assert_eq!(au64.get(), 1);
    }
}
