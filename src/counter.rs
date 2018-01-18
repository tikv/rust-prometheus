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

use desc::Desc;
use errors::{Error, Result};
use metrics::{Collector, Metric, Opts};
use proto;
use std::sync::Arc;
use value::{Value, ValueType};
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
        Counter::with_opts_and_label_values(&opts, &[])
    }

    fn with_opts_and_label_values(opts: &Opts, label_values: &[&str]) -> Result<Counter> {
        let v = Value::new(opts, ValueType::Counter, 0.0, label_values)?;
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

    pub fn local(&self) -> LocalCounter {
        LocalCounter::new(self.clone())
    }
}

impl Collector for Counter {
    fn desc(&self) -> Vec<&Desc> {
        vec![&self.v.desc]
    }

    fn collect(&self) -> Vec<proto::MetricFamily> {
        vec![self.v.collect()]
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

    fn build(&self, opts: &Opts, vals: &[&str]) -> Result<Counter> {
        Counter::with_opts_and_label_values(opts, vals)
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
        let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
        let opts = opts.variable_labels(variable_names);
        let metric_vec = MetricVec::create(proto::MetricType::COUNTER, CounterVecBuilder {}, opts)?;

        Ok(metric_vec as CounterVec)
    }
}

pub struct LocalCounter {
    counter: Counter,
    val: f64,
}

// LocalCounter is a thread local copy of Counter
impl LocalCounter {
    pub fn new(counter: Counter) -> LocalCounter {
        LocalCounter {
            counter: counter,
            val: 0.0,
        }
    }

    /// `inc_by` increments the given value to the local counter. Error if the value is <
    /// 0.
    #[inline]
    pub fn inc_by(&mut self, v: f64) -> Result<()> {
        if v < 0.0 {
            return Err(Error::DecreaseCounter(v));
        }
        self.val += v;
        Ok(())
    }

    /// `inc` increments the local counter by 1.
    #[inline]
    pub fn inc(&mut self) {
        self.val += 1.0;
    }

    /// `get` returns the local counter value.
    #[inline]
    pub fn get(&self) -> f64 {
        self.val
    }

    /// `flush` the local counter value to the counter
    #[inline]
    pub fn flush(&mut self) {
        if self.val == 0.0 {
            return;
        }
        self.counter.inc_by(self.val).unwrap();
        self.val = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use metrics::{Collector, Opts};
    use std::collections::HashMap;

    #[test]
    fn test_counter() {
        let opts = Opts::new("test", "test help")
            .const_label("a", "1")
            .const_label("b", "2");
        let counter = Counter::with_opts(opts).unwrap();
        counter.inc();
        assert_eq!(counter.get() as u64, 1);
        counter.inc_by(42.0).unwrap();
        assert_eq!(counter.get() as u64, 43);

        let mut mfs = counter.collect();
        assert_eq!(mfs.len(), 1);

        let mf = mfs.pop().unwrap();
        let m = mf.get_metric().get(0).unwrap();
        assert_eq!(m.get_label().len(), 2);
        assert_eq!(m.get_counter().get_value() as u64, 43);
    }

    #[test]
    fn test_local_counter() {
        let counter = Counter::new("counter", "counter helper").unwrap();
        let mut local_counter1 = counter.local();
        let mut local_counter2 = counter.local();

        local_counter1.inc();
        local_counter2.inc();
        assert_eq!(local_counter1.get() as u64, 1);
        assert_eq!(local_counter2.get() as u64, 1);
        assert_eq!(counter.get() as u64, 0);
        local_counter1.flush();
        assert_eq!(counter.get() as u64, 1);
        local_counter2.flush();
        assert_eq!(counter.get() as u64, 2);
    }

    #[test]
    fn test_counter_vec_with_labels() {
        let vec = CounterVec::new(
            Opts::new("test_couter_vec", "test counter vec help"),
            &["l1", "l2"],
        ).unwrap();

        let mut labels = HashMap::new();
        labels.insert("l1", "v1");
        labels.insert("l2", "v2");
        assert!(vec.remove(&labels).is_err());

        vec.with(&labels).inc();
        assert!(vec.remove(&labels).is_ok());
        assert!(vec.remove(&labels).is_err());

        let mut labels2 = HashMap::new();
        labels2.insert("l1", "v2");
        labels2.insert("l2", "v1");

        vec.with(&labels).inc();
        assert!(vec.remove(&labels2).is_err());

        vec.with(&labels).inc();

        let mut labels3 = HashMap::new();
        labels3.insert("l1", "v1");
        assert!(vec.remove(&labels3).is_err());
    }

    #[test]
    fn test_counter_vec_with_label_values() {
        let vec = CounterVec::new(
            Opts::new("test_vec", "test counter vec help"),
            &["l1", "l2"],
        ).unwrap();

        assert!(vec.remove_label_values(&["v1", "v2"]).is_err());
        vec.with_label_values(&["v1", "v2"]).inc();
        assert!(vec.remove_label_values(&["v1", "v2"]).is_ok());

        vec.with_label_values(&["v1", "v2"]).inc();
        assert!(vec.remove_label_values(&["v1"]).is_err());
        assert!(vec.remove_label_values(&["v1", "v3"]).is_err());
    }
}
