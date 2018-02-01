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

mod counter;
mod local_counter_primitive;
mod local_counter;

use errors::{Error, Result};

pub use self::counter::Counter;
pub use self::local_counter_primitive::LocalCounterPrimitive;

pub trait ICounter {
    /// `try_inc_by` increments the given value to the counter. Error if the value is < 0.
    fn try_inc_by(&mut self, v: f64) -> Result<()>;

    /// `inc_by` increments the given value to the counter. Panics if the value is < 0.
    #[inline]
    fn inc_by(&mut self, v: f64) {
        self.try_inc_by(v).unwrap();
    }

    /// `inc` increments the counter by 1.
    #[inline]
    fn inc(&mut self) {
        self.inc_by(1.0);
    }

    /// `get` returns the counter value.
    fn get(&self) -> f64;

    /// Reset any temporal value this counter is storing, if applicable.
    fn reset(&mut self);

    /// Merge current counter value into another compatible counter.
    #[inline]
    fn merge_to<T: ICounter>(&self, another: &mut T) {
        another.inc_by(self.get());
    }

    /// Merge current counter value into another compatible counter and clear current value.
    #[inline]
    fn flush_to<T: ICounter>(&mut self, another: &mut T) {
        self.merge_to(another);
        self.reset();
    }
}
