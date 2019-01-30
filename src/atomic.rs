use std::sync::atomic;

pub trait Number: Copy + Sized {
    fn is_not_negative(self) -> bool;

    fn zero() -> Self;
}

impl Number for isize {
    #[inline]
    fn is_not_negative(self) -> bool {
        self >= 0
    }

    #[inline]
    fn zero() -> isize {
        0
    }
}

pub trait Atomic: Send + Sync + Sized {
    /// The numeric type associated with this atomic.
    type Num: Number;

    /// Creates a new atomic value.
    fn new(val: Self::Num) -> Self;

    /// Creates a new atomic that value is 0.
    fn zero() -> Self {
        Self::new(Self::Num::zero())
    }

    /// Sets the value.
    fn set(&self, val: Self::Num);

    /// Gets the value.
    fn get(&self) -> Self::Num;

    /// Increases the value by a given amount.
    fn inc_by(&self, delta: Self::Num);

    /// Decreases the value by a given amount.
    fn dec_by(&self, delta: Self::Num);

    /// Increases the value.
    fn inc(&self);

    /// Decreases the value.
    fn dec(&self);
}

pub struct AtomicInt(atomic::AtomicIsize);

impl Atomic for AtomicInt {
    type Num = isize;

    #[inline]
    fn new(val: Self::Num) -> Self {
        Self(atomic::AtomicIsize::new(val))
    }

    #[inline]
    fn set(&self, val: Self::Num) {
        self.0.store(val, atomic::Ordering::Relaxed);
    }

    #[inline]
    fn get(&self) -> Self::Num {
        self.0.load(atomic::Ordering::Relaxed)
    }

    #[inline]
    fn inc_by(&self, delta: Self::Num) {
        self.0.fetch_add(delta, atomic::Ordering::Relaxed);
    }

    #[inline]
    fn dec_by(&self, delta: Self::Num) {
        self.0.fetch_sub(delta, atomic::Ordering::Relaxed);
    }

    #[inline]
    fn inc(&self) { self.inc_by(1) }

    #[inline]
    fn dec(&self) { self.dec_by(1) }
}
