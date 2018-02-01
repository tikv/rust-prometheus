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

use super::*;

#[derive(Clone)]
pub struct LocalCounter {
    counter: Counter,
    primitive: LocalCounterPrimitive,
}

// LocalCounter is a thread local copy of Counter
impl LocalCounter {
    pub fn new(counter: Counter) -> LocalCounter {
        LocalCounter {
            counter,
            primitive: LocalCounterPrimitive::new(),
        }
    }

    /// Flush the metrics to the associated non-local Counter.
    #[inline]
    pub fn flush(&mut self) {
        self.primitive.flush_to(self.counter);
    }
}

impl ICounter for LocalCounter {
    #[inline]
    fn try_inc_by(&mut self, v: f64) -> Result<()> {
        self.primitive.try_inc_by(v)
    }

    #[inline]
    fn get(&self) -> f64 {
        self.primitive.get()
    }

    #[inline]
    fn reset(&mut self) {
        self.primitive.reset()
    }
}
