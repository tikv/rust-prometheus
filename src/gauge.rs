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
use desc::Desc;
use errors::Result;
use value::{Value, ValueType};
use metrics::{Opts, Collector, Metric};
use vec::{MetricVec, MetricVecBuilder};

/// `Gauge` is a Metric that represents a single numerical value that can
/// arbitrarily go up and down.
#[derive(Clone)]
pub struct Gauge {
    v: Arc<Value>,
}

impl Gauge {
    pub fn new<S: Into<String>>(name: S, help: S) -> Result<Gauge> {
        let opts = Opts::new(name, help);
        Gauge::with_opts(opts)
    }

    pub fn with_opts(opts: Opts) -> Result<Gauge> {
        let desc = try!(Desc::new(opts.fq_name(), opts.help, vec![], opts.const_labels));
        Gauge::with_desc(desc, &[])
    }

    fn with_desc(desc: Desc, label_values: &[&str]) -> Result<Gauge> {
        let v = try!(Value::new(desc, ValueType::Gauge, 0.0, label_values));
        Ok(Gauge { v: Arc::new(v) })
    }
}

impl Gauge {
    // `set` sets the gauge to an arbitrary value.
    #[inline]
    pub fn set(&self, v: f64) {
        self.v.set(v);
    }

    /// `inc` increments the gauge by 1.
    #[inline]
    pub fn inc(&self) {
        self.add(1.0);
    }

    /// `dec` decrements the gauge by 1.
    #[inline]
    pub fn dec(&self) {
        self.sub(1.0);
    }

    // `add` adds the given value to the gauge. (The value can be
    // negative, resulting in a decrease of the gauge.)
    #[inline]
    pub fn add(&self, v: f64) {
        self.v.inc_by(v);
    }

    // `sub` subtracts the given value from the gauge. (The value can be
    // negative, resulting in an increase of the gauge.)
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
    fn desc(&self) -> &Desc {
        &self.v.desc
    }

    fn collect(&self) -> proto::MetricFamily {
        self.v.collect()
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
    type Output = Gauge;

    fn build(&self, desc: &Desc, vals: &[&str]) -> Result<Gauge> {
        Gauge::with_desc(desc.clone(), vals)
    }
}

/// `GaugeVec` is a Collector that bundles a set of Gauges that all share the same
/// Desc, but have different values for their variable labels. This is used if
/// you want to count the same thing partitioned by various dimensions
/// (e.g. number of operations queued, partitioned by user and operation type).
pub type GaugeVec = MetricVec<GaugeVecBuilder>;

impl GaugeVec {
    pub fn new(opts: Opts, label_names: &[&str]) -> Result<GaugeVec> {
        let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
        let desc = try!(Desc::new(opts.fq_name(), opts.help, variable_names, opts.const_labels));
        let metric_vec = MetricVec::create(desc, proto::MetricType::GAUGE, GaugeVecBuilder {});

        Ok(metric_vec as GaugeVec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use metrics::{Opts, Collector};

    #[test]
    fn test_gauge() {
        let opts = Opts::new("test", "test help").const_label("a", "1").const_label("b", "2");
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

        let mf = gauge.collect();
        let m = mf.get_metric().as_ref().get(0).unwrap();
        assert_eq!(m.get_label().len(), 2);
        assert_eq!(m.get_gauge().get_value() as u64, 42);
    }
}
