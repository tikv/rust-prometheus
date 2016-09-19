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

#[cfg(not(feature = "nightly"))]
use std::sync::RwLock;
#[cfg(feature = "nightly")]
use std::sync::atomic::{AtomicU64, Ordering};

pub struct AtomicF64 {
    #[cfg(not(feature = "nightly"))]
    inner: RwLock<f64>,

    #[cfg(feature = "nightly")]
    inner: AtomicU64,
}

#[cfg(not(feature = "nightly"))]
impl AtomicF64 {
    pub fn new(val: f64) -> AtomicF64 {
        AtomicF64 { inner: RwLock::new(val) }
    }

    #[inline]
    pub fn set(&self, val: f64) {
        *(self.inner).write().unwrap() = val;
    }

    #[inline]
    pub fn get(&self) -> f64 {
        *(self.inner).read().unwrap()
    }

    #[inline]
    pub fn inc_by(&self, delta: f64) {
        *(self.inner).write().unwrap() += delta;
    }
}

#[cfg(feature = "nightly")]
impl AtomicF64 {
    pub fn new(val: f64) -> AtomicF64 {
        AtomicF64 { inner: AtomicU64::new(val.as_u64()) }
    }

    #[inline]
    pub fn get(&self) -> f64 {
        self.inner.load(Ordering::Acquire).as_f64()
    }

    #[inline]
    pub fn set(&self, val: f64) {
        self.inner.store(val.as_u64(), Ordering::Release)
    }

    #[inline]
    pub fn inc_by(&self, delta: f64) {
        loop {
            let current = self.inner.load(Ordering::Acquire);
            let new = current.as_f64() + delta;
            let swapped = self.inner.compare_and_swap(current.as_u64(), new.as_u64(), Ordering::Release);
            if swapped == current {
                return;
            }
        }
    }
}

#[cfg(feature = "nightly")]
trait Transform64bits: Copy {
    fn as_u64(&self) -> u64;

    fn as_f64(&self) -> f64;
}

#[cfg(feature = "nightly")]
impl Transform64bits for u64 {
    fn as_u64(&self) -> u64 {
        *self
    }

    fn as_f64(&self) -> f64 {
        unsafe { *((self as *const u64) as *const f64) }
    }
}

#[cfg(feature = "nightly")]
impl Transform64bits for f64 {
    fn as_u64(&self) -> u64 {
        unsafe { *((self as *const f64) as *const u64) }
    }

    fn as_f64(&self) -> f64 {
        *self
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;
    use std::f64::{self, EPSILON};

    use super::*;

    #[cfg(feature = "nightly")]
    #[test]
    fn test_atomic_atomicf64() {
        let table: Vec<f64> = vec![0.0, 1.0, PI, f64::MIN, f64::MAX];

        for f in table {
            assert!((f - AtomicF64::new(f).get()).abs() < EPSILON);
        }
    }

    #[cfg(not(feature = "nightly"))]
    #[test]
    fn test_rwlock_atomicf64() {
        let table: Vec<f64> = vec![0.0, 1.0, PI, f64::MIN, f64::MAX];

        for f in table {
            assert!((f - AtomicF64::new(f).get()).abs() < EPSILON);
        }
    }
}
