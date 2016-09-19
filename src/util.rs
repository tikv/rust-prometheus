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

pub fn f64u64(f: f64) -> u64 {
    unsafe { *((&f as *const f64) as *const u64) }
}

pub fn u64f64(u: u64) -> f64 {
    unsafe { *((&u as *const u64) as *const f64) }
}

#[cfg(test)]
mod test {
    use std::f64::{self, EPSILON};
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_from_f64_to_u64_and_back() {
        let table: Vec<f64> = vec![0.0, 1.0, PI, f64::MIN, f64::MAX];

        for f in table {
            assert!((f - u64f64(f64u64(f))).abs() < EPSILON);
        }
    }
}
