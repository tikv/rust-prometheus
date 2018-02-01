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

use errors::{Result, Error};
use super::*;

#[derive(Clone, Debug)]
pub struct LocalCounterPrimitive {
    val: f64,
}

impl LocalCounterPrimitive {
    pub fn new() -> LocalCounterPrimitive {
        LocalCounterPrimitive {
            val: 0.0,
        }
    }
}

impl ICounter for LocalCounterPrimitive {
    fn try_inc_by(&mut self, v: f64) -> Result<()> {
        if v < 0.0 {
            return Err(Error::DecreaseCounter(v));
        }
        self.val += v;
        Ok(())
    }

    fn get(&self) -> f64 {
        self.val
    }

    fn reset(&mut self) {
        self.val = 0.0;
    }
}
