// Copyright 2014 The Prometheus Authors
// Copyright 2018 PingCAP, Inc.
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
mod fallback;
#[cfg(not(feature = "nightly"))]
pub use self::fallback::{AtomicF64, AtomicU64};

#[cfg(feature = "nightly")]
mod nightly;
#[cfg(feature = "nightly")]
pub use self::nightly::{AtomicF64, AtomicU64};

pub trait Atomic<T> {
    fn new(val: T) -> Self;
    fn set(&self, val: T);
    fn get(&self) -> T;
    fn inc_by(&self, delta: T);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::{self, EPSILON};
    use std::f64::consts::PI;

    #[test]
    fn test_atomic_f64() {
        let table: Vec<f64> = vec![0.0, 1.0, PI, f64::MIN, f64::MAX];

        for f in table {
            assert!((f - AtomicF64::new(f).get()).abs() < EPSILON);
        }
    }

    #[test]
    fn test_atomic_u64() {
        let au64 = AtomicU64::new(0);
        assert_eq!(au64.get(), 0);

        au64.inc_by(1);
        assert_eq!(au64.get(), 1);
    }
}
