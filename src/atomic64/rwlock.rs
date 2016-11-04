use std::sync::RwLock;

pub struct F64 {
    inner: RwLock<f64>,
}

impl F64 {
    pub fn new(val: f64) -> F64 {
        F64 { inner: RwLock::new(val) }
    }

    #[inline]
    pub fn set(&self, val: f64) {
        *self.inner.write().unwrap() = val;
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

pub struct U64 {
    inner: RwLock<u64>,
}

impl U64 {
    pub fn new(val: u64) -> U64 {
        U64 { inner: RwLock::new(val) }
    }

    #[inline]
    pub fn get(&self) -> u64 {
        *self.inner.read().unwrap()
    }

    #[inline]
    pub fn inc_by(&self, delta: u64) {
        *self.inner.write().unwrap() += delta;
    }
}
