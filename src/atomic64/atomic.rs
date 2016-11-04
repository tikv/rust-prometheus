use std::sync::atomic::Ordering;
use std::mem::transmute;

pub struct F64 {
    inner: super::AtomicU64Type,
}

impl F64 {
    pub fn new(val: f64) -> F64 {
        F64 { inner: super::AtomicU64Type::new(f64_to_u64(val)) }
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

fn u64_to_f64(val: super::U64Type) -> f64 {
    unsafe { transmute(val) }
}

fn f64_to_u64(val: f64) -> super::U64Type {
    unsafe { transmute(val) }
}

pub struct U64 {
    inner: super::AtomicU64Type,
}

impl U64 {
    pub fn new(val: u64) -> U64 {
        U64 { inner: super::AtomicU64Type::new(val as super::U64Type) }
    }

    #[inline]
    pub fn get(&self) -> u64 {
        self.inner.load(Ordering::Acquire) as u64
    }

    #[inline]
    pub fn inc_by(&self, delta: u64) {
        self.inner.fetch_add(delta as super::U64Type, Ordering::Release);
    }
}
