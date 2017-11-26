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


#[cfg(feature = "nightly")]
pub use self::atomic::{AtomicF64, AtomicU64};
#[cfg(not(feature = "nightly"))]
pub use self::rwlock::{RwlockF64 as AtomicF64, RwlockU64 as AtomicU64};

#[cfg(not(feature = "nightly"))]
mod rwlock {
    use spin::RwLock;

    pub struct RwlockF64 {
        inner: RwLock<f64>,
    }

    impl RwlockF64 {
        pub fn new(val: f64) -> RwlockF64 {
            RwlockF64 {
                inner: RwLock::new(val),
            }
        }

        #[inline]
        pub fn set(&self, val: f64) {
            *self.inner.write() = val;
        }

        #[inline]
        pub fn get(&self) -> f64 {
            *self.inner.read()
        }

        #[inline]
        pub fn inc_by(&self, delta: f64) {
            *self.inner.write() += delta;
        }
    }

    pub struct RwlockU64 {
        inner: RwLock<u64>,
    }

    impl RwlockU64 {
        pub fn new(val: u64) -> RwlockU64 {
            RwlockU64 {
                inner: RwLock::new(val),
            }
        }

        #[inline]
        pub fn get(&self) -> u64 {
            *self.inner.read()
        }

        #[inline]
        pub fn inc_by(&self, delta: u64) {
            *self.inner.write() += delta;
        }
    }
}

#[cfg(feature = "nightly")]
mod atomic {
    use std::f64;
    use std::sync::atomic::{AtomicU64 as StdAtomicU64, Ordering};

    pub struct AtomicF64 {
        inner: StdAtomicU64,
    }

    impl AtomicF64 {
        pub fn new(val: f64) -> AtomicF64 {
            AtomicF64 {
                inner: StdAtomicU64::new(f64_to_u64(val)),
            }
        }

        #[inline]
        pub fn get(&self) -> f64 {
            u64_to_f64(self.inner.load(Ordering::Relaxed))
        }

        #[inline]
        pub fn set(&self, val: f64) {
            self.inner.store(f64_to_u64(val), Ordering::Relaxed)
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
        f64::from_bits(val)
    }

    fn f64_to_u64(val: f64) -> u64 {
        f64::to_bits(val)
    }

    pub struct AtomicU64 {
        inner: StdAtomicU64,
    }

    impl AtomicU64 {
        pub fn new(val: u64) -> AtomicU64 {
            AtomicU64 {
                inner: StdAtomicU64::new(val),
            }
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
}

#[cfg(test)]
mod test {

    use super::*;
    use std::f64::{self, EPSILON};
    use std::f64::consts::PI;

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
