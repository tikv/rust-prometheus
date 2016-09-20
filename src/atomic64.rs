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

use atomic::{Atomic, Ordering};

pub struct AtomicF64 {
    inner: Atomic<f64>,
}

impl AtomicF64 {
    pub fn new(val: f64) -> AtomicF64 {
        AtomicF64 { inner: Atomic::new(val) }
    }

    #[inline]
    pub fn get(&self) -> f64 {
        self.inner.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn set(&self, val: f64) {
        self.inner.store(val, Ordering::Release)
    }

    #[inline]
    pub fn inc_by(&self, delta: f64) {
        loop {
            let current = self.inner.load(Ordering::Acquire);
            let new = current + delta;
            if let Ok(_) = self.inner
                .compare_exchange(current, new, Ordering::Release, Ordering::Release) {
                return;
            }
        }
    }
}

pub struct AtomicU64 {
    inner: Atomic<u64>,
}

impl AtomicU64 {
    pub fn new(val: u64) -> AtomicU64 {
        AtomicU64 { inner: Atomic::new(val) }
    }

    #[inline]
    pub fn get(&self) -> u64 {
        self.inner.load(Ordering::Acquire)
    }

    #[inline]
    pub fn inc_by(&self, delta: u64) {
        self.inner.fetch_add(delta, Ordering::Release);
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
