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

use std::f64;
use std::sync::atomic::{AtomicU64 as StdAtomicU64, Ordering};

use super::Atomic;

pub struct AtomicF64 {
    inner: StdAtomicU64,
}

#[inline]
fn u64_to_f64(val: u64) -> f64 {
    f64::from_bits(val)
}

#[inline]
fn f64_to_u64(val: f64) -> u64 {
    f64::to_bits(val)
}

impl Atomic<f64> for AtomicF64 {
    fn new(val: f64) -> AtomicF64 {
        AtomicF64 {
            inner: StdAtomicU64::new(f64_to_u64(val)),
        }
    }

    #[inline]
    fn set(&self, val: f64) {
        self.inner.store(f64_to_u64(val), Ordering::Relaxed);
    }

    #[inline]
    fn get(&self) -> f64 {
        u64_to_f64(self.inner.load(Ordering::Relaxed))
    }

    #[inline]
    fn inc_by(&self, delta: f64) {
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

pub struct AtomicU64 {
    inner: StdAtomicU64,
}

impl Atomic<u64> for AtomicU64 {
    fn new(val: u64) -> AtomicU64 {
        AtomicU64 {
            inner: StdAtomicU64::new(val),
        }
    }

    #[inline]
    fn set(&self, val: u64) {
        self.inner.store(val, Ordering::Relaxed);
    }

    #[inline]
    fn get(&self) -> u64 {
        self.inner.load(Ordering::Acquire)
    }

    #[inline]
    fn inc_by(&self, delta: u64) {
        self.inner.fetch_add(delta, Ordering::Release);
    }
}
