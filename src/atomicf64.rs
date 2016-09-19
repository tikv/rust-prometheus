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
pub use self::rwlock::RwlockF64 as AtomicF64;
#[cfg(feature = "nightly")]
pub use self::atomic::AtomicF64;

#[cfg(not(feature = "nightly"))]
mod rwlock {
    use std::sync::RwLock;

    pub struct RwlockF64 {
        inner: RwLock<f64>,
    }

    impl RwlockF64 {
        pub fn new(val: f64) -> RwlockF64 {
            RwlockF64 { inner: RwLock::new(val) }
        }

        #[inline]
        pub fn set(&self, val: f64) {
            *(self.inner).write().unwrap() = val;
        }

        #[inline]
        pub fn get(&self) -> f64 {
            *self.inner.read().unwrap()
        }

        #[inline]
        pub fn inc_by(&self, delta: f64) {
            *self.inner.write().unwrap() += delta;
        }
    }
}

#[cfg(feature = "nightly")]
mod atomic {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::mem::transmute;

    pub struct AtomicF64 {
        inner: AtomicU64,
    }

    impl AtomicF64 {
        pub fn new(val: f64) -> AtomicF64 {
            AtomicF64 { inner: AtomicU64::new(f64_to_u64(val)) }
        }

        #[inline]
        pub fn get(&self) -> f64 {
            u64_to_f64(self.inner.load(Ordering::Relaxed))
        }

        #[inline]
        pub fn set(&self, val: f64) {
            self.inner.store(f64_to_u64(val), Ordering::Release)
        }

        #[inline]
        pub fn inc_by(&self, delta: f64) {
            loop {
                let current = self.inner.load(Ordering::Acquire);
                let new = u64_to_f64(current) + delta;
                let swapped = self.inner
                    .compare_and_swap(current, f64_to_u64(new), Ordering::Release);
                if swapped == current {
                    return;
                }
            }
        }
    }

    fn u64_to_f64(val: u64) -> f64 {
        unsafe { transmute(val) }
    }

    fn f64_to_u64(val: f64) -> u64 {
        unsafe { transmute(val) }
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
}
