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

use std::sync::Arc;

use proto;
use metrics::{Opts, Collector};
use value::{Value, ValueType};
use desc::Desc;
use errors::{Result, Error};

/// `Counter` is a Metric that represents a single numerical value that only ever
/// goes up.
#[derive(Clone)]
pub struct Counter {
    v: Arc<Value>,
}

impl Counter {
    pub fn new<S: Into<String>>(name: S, help: S) -> Result<Counter> {
        let opts = Opts::new(name, help);
        Counter::with_opts(opts)
    }

    pub fn with_opts(opts: Opts) -> Result<Counter> {
        let desc = try!(Desc::new(opts.fq_name(), opts.help, vec![], opts.const_labels));
        let v = try!(Value::new(desc, ValueType::Counter, 0.0, vec![]));
        Ok(Counter { v: Arc::new(v) })
    }
}

impl Counter {
    /// `inc_by` increments the given value to the counter. Error if the value is <
    /// 0.
    #[inline]
    pub fn inc_by(&self, v: f64) -> Result<()> {
        if v < 0.0 {
            return Err(Error::DecreaseCounter(v));
        }

        Ok(self.v.inc_by(v))
    }

    /// `inc` increments the counter by 1.
    #[inline]
    pub fn inc(&self) {
        self.inc_by(1.0).unwrap()
    }

    /// `get` returns the counter value.
    #[inline]
    pub fn get(&self) -> f64 {
        self.v.get()
    }
}

impl Collector for Counter {
    fn desc(&self) -> &Desc {
        &self.v.desc
    }

    fn collect(&self) -> proto::MetricFamily {
        self.v.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use metrics::{Opts, Collector};

    #[test]
    fn test_counter() {
        let opts = Opts::new("test", "test help").const_label("a", "1").const_label("b", "2");
        let counter = Counter::with_opts(opts).unwrap();
        counter.inc();
        assert_eq!(counter.get() as u64, 1);
        counter.inc_by(42.0).unwrap();
        assert_eq!(counter.get() as u64, 43);

        let mf = counter.collect();
        let m = mf.get_metric().as_ref().get(0).unwrap();
        assert_eq!(m.get_label().len(), 2);
        assert_eq!(m.get_counter().get_value() as u64, 43);
    }
}
