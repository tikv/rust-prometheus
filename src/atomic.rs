use std::sync::atomic;

/// An interface for number types that used in float collectors and integer collectors.
pub trait Number: 'static + Copy + Sized + Send + Sync {
    /// Returns the zero value.
    fn zero() -> Self;

    /// Whether the value is >= 0;
    fn is_not_negative(self) -> bool;
}

impl Number for isize {
    #[inline]
    fn zero() -> isize {
        0
    }

    #[inline]
    fn is_not_negative(self) -> bool {
        self >= 0
    }
}

impl Number for f64 {
    #[inline]
    fn zero() -> f64 {
        0f64
    }

    #[inline]
    fn is_not_negative(self) -> bool {
        self >= 0f64
    }
}

/// A trait for atomic number types, used to support float collectors and integer collectors.
pub trait Atomic: Send + Sync + Sized + 'static {
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

/// An atomic signed integer. Lock-free.
pub struct AtomicInt(atomic::AtomicIsize);

impl AtomicInt {
    #[inline]
    const fn new(val: isize) -> Self {
        Self(atomic::AtomicIsize::new(val))
    }
}

impl Atomic for AtomicInt {
    type Num = isize;

    #[inline]
    fn new(val: Self::Num) -> Self {
        AtomicInt::new(val)
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
    fn inc(&self) {
        self.inc_by(1)
    }

    #[inline]
    fn dec(&self) {
        self.dec_by(1)
    }
}

/// An atomic double-precision floating point. It is not lock-free when performing increment
/// or decrement.
pub struct AtomicFloat(atomic::AtomicU64);

impl AtomicFloat {
    #[inline]
    fn u64_to_f64(val: u64) -> f64 {
        // TODO: Mark as const fn
        f64::from_bits(val)
    }

    #[inline]
    fn f64_to_u64(val: f64) -> u64 {
        // TODO: Mark as const fn
        f64::to_bits(val)
    }

    #[inline]
    fn new(val: f64) -> Self {
        // TODO: Mark as const fn
        Self(atomic::AtomicU64::new(Self::f64_to_u64(val)))
    }
}

impl Atomic for AtomicFloat {
    type Num = f64;

    #[inline]
    fn new(val: Self::Num) -> Self {
        AtomicFloat::new(val)
    }

    #[inline]
    fn set(&self, val: Self::Num) {
        self.0
            .store(Self::f64_to_u64(val), atomic::Ordering::Relaxed);
    }

    #[inline]
    fn get(&self) -> Self::Num {
        Self::u64_to_f64(self.0.load(atomic::Ordering::Relaxed))
    }

    #[inline]
    fn inc_by(&self, delta: Self::Num) {
        loop {
            let current = self.0.load(atomic::Ordering::Acquire);
            let new = Self::u64_to_f64(current) + delta;
            let swapped =
                self.0
                    .compare_and_swap(current, Self::f64_to_u64(new), atomic::Ordering::Release);
            if swapped == current {
                return;
            }
        }
    }

    #[inline]
    fn dec_by(&self, delta: Self::Num) {
        self.inc_by(-delta);
    }

    #[inline]
    fn inc(&self) {
        self.inc_by(1f64);
    }

    #[inline]
    fn dec(&self) {
        self.dec_by(1f64);
    }
}
