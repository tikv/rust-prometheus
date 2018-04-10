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

use atomic64::{Atomic, AtomicF64, AtomicI64, Number};
use desc::Desc;
use errors::Result;
use metrics::{Collector, Metric, Opts};
use proto;
use std::marker::PhantomData;
use std::sync::Arc;
use value::{Value, ValueType};
use vec::{MetricVec, MetricVecBuilder};

/// The underlying implementation for [`Gauge`](::Gauge) and [`IntGauge`](::IntGauge).
pub struct GenericGauge<P: Atomic> {
    v: Arc<Value<P>>,
}

/// A [`Metric`](::core::Metric) represents a single numerical value that can arbitrarily go up and down.
pub type Gauge = GenericGauge<AtomicF64>;

/// The integer version of [`Gauge`](::Gauge). Provides better performance if metric values are all integers.
pub type IntGauge = GenericGauge<AtomicI64>;

impl<P: Atomic> Clone for GenericGauge<P> {
    fn clone(&self) -> Self {
        Self {
            v: Arc::clone(&self.v),
        }
    }
}

impl<P: Atomic> GenericGauge<P> {
    /// Create a [`GenericGauge`](::core::GenericGauge) with the `name` and `help` arguments.
    pub fn new<S: Into<String>>(name: S, help: S) -> Result<Self> {
        let opts = Opts::new(name, help);
        Self::with_opts(opts)
    }

    /// Create a [`GenericGauge`](::core::GenericGauge) with the `opts` options.
    pub fn with_opts(opts: Opts) -> Result<Self> {
        Self::with_opts_and_label_values(&opts, &[])
    }

    fn with_opts_and_label_values(opts: &Opts, label_values: &[&str]) -> Result<Self> {
        let v = Value::new(opts, ValueType::Gauge, P::T::from_i64(0), label_values)?;
        Ok(Self { v: Arc::new(v) })
    }

    /// Set the gauge to an arbitrary value.
    #[inline]
    pub fn set(&self, v: P::T) {
        self.v.set(v);
    }

    /// Increase the gauge by 1.
    #[inline]
    pub fn inc(&self) {
        self.v.inc();
    }

    /// Decrease the gauge by 1.
    #[inline]
    pub fn dec(&self) {
        self.v.dec();
    }

    /// Add the given value to the gauge. (The value can be
    /// negative, resulting in a decrement of the gauge.)
    #[inline]
    pub fn add(&self, v: P::T) {
        self.v.inc_by(v);
    }

    /// Subtract the given value from the gauge. (The value can be
    /// negative, resulting in an increment of the gauge.)
    #[inline]
    pub fn sub(&self, v: P::T) {
        self.v.dec_by(v);
    }

    /// Return the gauge value.
    #[inline]
    pub fn get(&self) -> P::T {
        self.v.get()
    }
}

impl<P: Atomic> Collector for GenericGauge<P> {
    fn desc(&self) -> Vec<&Desc> {
        vec![&self.v.desc]
    }

    fn collect(&self) -> Vec<proto::MetricFamily> {
        vec![self.v.collect()]
    }
}

impl<P: Atomic> Metric for GenericGauge<P> {
    fn metric(&self) -> proto::Metric {
        self.v.metric()
    }
}

pub struct GaugeVecBuilder<P: Atomic> {
    _phantom: PhantomData<P>,
}

impl<P: Atomic> GaugeVecBuilder<P> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<P: Atomic> Clone for GaugeVecBuilder<P> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<P: Atomic> MetricVecBuilder for GaugeVecBuilder<P> {
    type M = GenericGauge<P>;
    type P = Opts;

    fn build(&self, opts: &Opts, vals: &[&str]) -> Result<Self::M> {
        Self::M::with_opts_and_label_values(opts, vals)
    }
}

/// The underlying implementation for [`GaugeVec`](::GaugeVec) and [`IntGaugeVec`](::IntGaugeVec).
pub type GenericGaugeVec<P> = MetricVec<GaugeVecBuilder<P>>;

/// A [`Collector`](::core::Collector) that bundles a set of [`Gauge`](::Gauge)s that all share the same
/// Desc, but have different values for their variable labels. This is used if
/// you want to count the same thing partitioned by various dimensions
/// (e.g. number of operations queued, partitioned by user and operation type).
pub type GaugeVec = GenericGaugeVec<AtomicF64>;

/// The integer version of [`GaugeVec`](::GaugeVec). Provides better performance if metric values are all integers.
pub type IntGaugeVec = GenericGaugeVec<AtomicI64>;

impl<P: Atomic> GenericGaugeVec<P> {
    /// Create a new [`GenericGaugeVec`](::core::GenericGaugeVec) based on the provided [`Opts`](::Opts) and
    /// partitioned by the given label names. At least one label name must be
    /// provided.
    pub fn new(opts: Opts, label_names: &[&str]) -> Result<Self> {
        let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
        let opts = opts.variable_labels(variable_names);
        let metric_vec = MetricVec::create(proto::MetricType::GAUGE, GaugeVecBuilder::new(), opts)?;

        Ok(metric_vec as Self)
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
