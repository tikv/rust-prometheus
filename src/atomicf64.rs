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

use std::sync::atomic::{AtomicU64, Ordering};

trait Transform64bits: Copy {
    fn as_u64(&self) -> u64;

    fn as_f64(&self) -> f64;
}

impl Transform64bits for u64 {
    fn as_u64(&self) -> u64 {
        *self
    }

    fn as_f64(&self) -> f64 {
        unsafe { *((self as *const u64) as *const f64) }
    }
}

impl Transform64bits for f64 {
    fn as_u64(&self) -> u64 {
        unsafe { *((self as *const f64) as *const u64) }
    }

    fn as_f64(&self) -> f64 {
        *self
    }
}

pub struct AtomicF64 {
    inner: AtomicU64,
}

impl AtomicF64 {
    pub fn new(val: f64) -> AtomicF64 {
        AtomicF64 { inner: AtomicU64::new(val.as_u64()) }
    }

    #[inline]
    pub fn store(&self, val: f64, order: Ordering) {
        self.inner.store(val.as_u64(), order)
    }

    #[inline]
    pub fn load(&self, order: Ordering) -> f64 {
        self.inner.load(order).as_f64()
    }

    #[inline]
    pub fn compare_and_swap(&self, current: f64, new: f64, order: Ordering) -> f64 {
        self.inner.compare_and_swap(current.as_u64(), new.as_u64(), order).as_f64()
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;
    use std::f64::{self, EPSILON};
    use std::sync::atomic::Ordering;

    use super::*;

    #[test]
    fn test_atomicf64() {
        let table: Vec<f64> = vec![0.0, 1.0, PI, f64::MIN, f64::MAX];

        for f in table {
            assert!((f - AtomicF64::new(f).load(Ordering::Relaxed)).abs() < EPSILON);
        }
    }
}
