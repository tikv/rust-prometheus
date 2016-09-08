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
use metrics::{Opts, Collector, Metric};
use value::{Value, ValueType};
use desc::Desc;
use errors::{Result, Error};
use vec::{MetricVec, MetricVecBuilder};

/// `Counter` is a Metric that represents a single numerical value that only ever
/// goes up.
#[derive(Clone)]
pub struct Counter {
    v: Arc<Value>,
}

impl Counter {
    /// `new` creates a `Counter` with the `name` and `help` arguments.
    pub fn new<S: Into<String>>(name: S, help: S) -> Result<Counter> {
        let opts = Opts::new(name, help);
        Counter::with_opts(opts)
    }

    /// `with_opts` creates a `Counter` with the `opts` options.
    pub fn with_opts(opts: Opts) -> Result<Counter> {
        let desc = try!(Desc::new(opts.fq_name(), opts.help, vec![], opts.const_labels));
        Counter::with_desc(desc, &[])
    }

    fn with_desc(desc: Desc, label_values: &[&str]) -> Result<Counter> {
        let v = try!(Value::new(desc, ValueType::Counter, 0.0, label_values));
        Ok(Counter { v: Arc::new(v) })
    }

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

impl Metric for Counter {
    fn metric(&self) -> proto::Metric {
        self.v.metric()
    }
}

#[derive(Clone)]
pub struct CounterVecBuilder {}

impl MetricVecBuilder for CounterVecBuilder {
    type M = Counter;
    type P = Opts;

    fn build(&self, desc: &Desc, _: &Opts, vals: &[&str]) -> Result<Counter> {
        Counter::with_desc(desc.clone(), vals)
    }
}

/// `CounterVec` is a Collector that bundles a set of Counters that all share the
/// same Desc, but have different values for their variable labels. This is used
/// if you want to count the same thing partitioned by various dimensions
/// (e.g. number of HTTP requests, partitioned by response code and method).
pub type CounterVec = MetricVec<CounterVecBuilder>;

impl CounterVec {
    /// `new` creates a new `CounterVec` based on the provided `Opts` and
    /// partitioned by the given label names. At least one label name must be
    /// provided.
    pub fn new(opts: Opts, label_names: &[&str]) -> Result<CounterVec> {
        let opts1 = opts.clone();
        let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
        let desc = try!(Desc::new(opts.fq_name(), opts.help, variable_names, opts.const_labels));
        let metric_vec = MetricVec::create(desc,
                                           proto::MetricType::COUNTER,
                                           CounterVecBuilder {},
                                           opts1);

        Ok(metric_vec as CounterVec)
    }
}

#[cfg(test)]
mod tests {
    use metrics::{Opts, Collector};

    use super::*;

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
