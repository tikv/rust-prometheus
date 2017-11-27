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
use errors::Result;
use metrics::{Collector, Metric, Opts};

use proto;
use std::sync::Arc;
use value::{Value, ValueType};
use vec::{MetricVec, MetricVecBuilder};

/// `Gauge` is a Metric that represents a single numerical value that can
/// arbitrarily go up and down.
#[derive(Clone)]
pub struct Gauge {
    v: Arc<Value>,
}

impl Gauge {
    /// `new` create a `Guage` with the `name` and `help` arguments.
    pub fn new<S: Into<String>>(name: S, help: S) -> Result<Gauge> {
        let opts = Opts::new(name, help);
        Gauge::with_opts(opts)
    }

    /// `with_opts` create a `Guage` with the `opts` options.
    pub fn with_opts(opts: Opts) -> Result<Gauge> {
        Gauge::with_opts_and_label_values(&opts, &[])
    }

    fn with_opts_and_label_values(opts: &Opts, label_values: &[&str]) -> Result<Gauge> {
        let v = Value::new(opts, ValueType::Gauge, 0.0, label_values)?;
        Ok(Gauge { v: Arc::new(v) })
    }
}

impl Gauge {
    /// `set` sets the gauge to an arbitrary value.
    #[inline]
    pub fn set(&self, v: f64) {
        self.v.set(v);
    }

    /// `inc` increments the gauge by 1.
    #[inline]
    pub fn inc(&self) {
        self.v.inc();
    }

    /// `dec` decrements the gauge by 1.
    #[inline]
    pub fn dec(&self) {
        self.v.dec();
    }

    /// `add` adds the given value to the gauge. (The value can be
    /// negative, resulting in a decrease of the gauge.)
    #[inline]
    pub fn add(&self, v: f64) {
        self.v.inc_by(v);
    }

    /// `sub` subtracts the given value from the gauge. (The value can be
    /// negative, resulting in an increase of the gauge.)
    #[inline]
    pub fn sub(&self, v: f64) {
        self.v.dec_by(v);
    }

    /// `get` returns the gauge value.
    #[inline]
    pub fn get(&self) -> f64 {
        self.v.get()
    }
}

impl Collector for Gauge {
    fn desc(&self) -> Vec<&Desc> {
        vec![&self.v.desc]
    }

    fn collect(&self) -> Vec<proto::MetricFamily> {
        vec![self.v.collect()]
    }
}

impl Metric for Gauge {
    fn metric(&self) -> proto::Metric {
        self.v.metric()
    }
}

#[derive(Clone)]
pub struct GaugeVecBuilder {}

impl MetricVecBuilder for GaugeVecBuilder {
    type M = Gauge;
    type P = Opts;

    fn build(&self, opts: &Opts, vals: &[&str]) -> Result<Gauge> {
        Gauge::with_opts_and_label_values(opts, vals)
    }
}

/// `GaugeVec` is a Collector that bundles a set of Gauges that all share the same
/// Desc, but have different values for their variable labels. This is used if
/// you want to count the same thing partitioned by various dimensions
/// (e.g. number of operations queued, partitioned by user and operation type).
pub type GaugeVec = MetricVec<GaugeVecBuilder>;

impl GaugeVec {
    /// `new` creates a new `GaugeVec` based on the provided `Opts` and
    /// partitioned by the given label names. At least one label name must be
    /// provided.
    pub fn new(opts: Opts, label_names: &[&str]) -> Result<GaugeVec> {
        let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
        let opts = opts.variable_labels(variable_names);
        let metric_vec = MetricVec::create(proto::MetricType::GAUGE, GaugeVecBuilder {}, opts)?;

        Ok(metric_vec as GaugeVec)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use metrics::{Collector, Opts};
    use std::collections::HashMap;

    #[test]
    fn test_gauge() {
        let opts = Opts::new("test", "test help")
            .const_label("a", "1")
            .const_label("b", "2");
        let gauge = Gauge::with_opts(opts).unwrap();
        gauge.inc();
        assert_eq!(gauge.get() as u64, 1);
        gauge.add(42.0);
        assert_eq!(gauge.get() as u64, 43);
        gauge.sub(42.0);
        assert_eq!(gauge.get() as u64, 1);
        gauge.dec();
        assert_eq!(gauge.get() as u64, 0);
        gauge.set(42.0);
        assert_eq!(gauge.get() as u64, 42);

        let mut mfs = gauge.collect();
        assert_eq!(mfs.len(), 1);

        let mf = mfs.pop().unwrap();
        let m = mf.get_metric().get(0).unwrap();
        assert_eq!(m.get_label().len(), 2);
        assert_eq!(m.get_gauge().get_value() as u64, 42);
    }

    #[test]
    fn test_gauge_vec_with_labels() {
        let vec = GaugeVec::new(
            Opts::new("test_gauge_vec", "test gauge vec help"),
            &["l1", "l2"],
        ).unwrap();

        let mut labels = HashMap::new();
        labels.insert("l1", "v1");
        labels.insert("l2", "v2");
        assert!(vec.remove(&labels).is_err());

        vec.with(&labels).inc();
        vec.with(&labels).dec();
        vec.with(&labels).add(42.0);
        vec.with(&labels).sub(42.0);
        vec.with(&labels).set(42.0);

        assert!(vec.remove(&labels).is_ok());
        assert!(vec.remove(&labels).is_err());
    }

    #[test]
    fn test_gauge_vec_with_label_values() {
        let vec = GaugeVec::new(
            Opts::new("test_gauge_vec", "test gauge vec help"),
            &["l1", "l2"],
        ).unwrap();

        assert!(vec.remove_label_values(&["v1", "v2"]).is_err());
        vec.with_label_values(&["v1", "v2"]).inc();
        assert!(vec.remove_label_values(&["v1", "v2"]).is_ok());

        vec.with_label_values(&["v1", "v2"]).inc();
        vec.with_label_values(&["v1", "v2"]).dec();
        vec.with_label_values(&["v1", "v2"]).add(42.0);
        vec.with_label_values(&["v1", "v2"]).sub(42.0);
        vec.with_label_values(&["v1", "v2"]).set(42.0);

        assert!(vec.remove_label_values(&["v1"]).is_err());
        assert!(vec.remove_label_values(&["v1", "v3"]).is_err());
    }

}
