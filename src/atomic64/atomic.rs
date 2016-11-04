use std::sync::atomic::{AtomicU64, Ordering};
use std::mem::transmute;

pub struct F64 {
    inner: AtomicU64,
}

impl F64 {
    pub fn new(val: f64) -> F64 {
        F64 { inner: AtomicU64::new(f64_to_u64(val)) }
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
    unsafe { transmute(val) }
}

fn f64_to_u64(val: f64) -> u64 {
    unsafe { transmute(val) }
}

pub struct U64 {
    inner: AtomicU64,
}

impl U64 {
    pub fn new(val: u64) -> U64 {
        U64 { inner: AtomicU64::new(val) }
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
