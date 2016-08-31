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

/// Untyped is a Metric that represents a single numerical value that can
/// arbitrarily go up and down.
///
/// An Untyped metric works the same as a Gauge. The only difference is that to
/// no type information is implied.
#[derive(Clone)]
pub struct Untyped {
    v: Arc<Value>,
}

impl Untyped {
    /// `new` creates a `Untyped` with the `name` and `help` arguments.
    pub fn new<S: Into<String>>(name: S, help: S) -> Result<Untyped> {
        let opts = Opts::new(name, help);
        Untyped::with_opts(opts)
    }

    /// `with_opts` creates a `Untyped` with the `opts` options.
    pub fn with_opts(opts: Opts) -> Result<Untyped> {
        let desc = try!(Desc::new(opts.fq_name(), opts.help, vec![], opts.const_labels));
        Untyped::with_desc(desc, &[])
    }

    fn with_desc(desc: Desc, label_values: &[&str]) -> Result<Untyped> {
        let v = try!(Value::new(desc, ValueType::Untyped, 0.0, label_values));
        Ok(Untyped { v: Arc::new(v) })
    }
}

impl Untyped {
    /// `set` sets the untyped to an arbitrary value.
    #[inline]
    pub fn set(&self, v: f64) {
        self.v.set(v);
    }

    /// `inc` increments the untyped by 1.
    #[inline]
    pub fn inc(&self) {
        self.add(1.0);
    }

    /// `dec` decrements the untyped by 1.
    #[inline]
    pub fn dec(&self) {
        self.sub(1.0);
    }

    /// `add` adds the given value to the untyped. (The value can be
    /// negative, resulting in a decrease.)
    #[inline]
    pub fn add(&self, v: f64) {
        self.v.inc_by(v);
    }

    /// `sub` subtracts the given value from the untyped. (The value can be
    /// negative, resulting in an increase.)
    #[inline]
    pub fn sub(&self, v: f64) {
        self.v.dec_by(v);
    }

    /// `get` returns the untyped value.
    #[inline]
    pub fn get(&self) -> f64 {
        self.v.get()
    }
}

impl Collector for Untyped {
    fn desc(&self) -> &Desc {
        &self.v.desc
    }

    fn collect(&self) -> proto::MetricFamily {
        self.v.collect()
    }
}

impl Metric for Untyped {
    fn metric(&self) -> proto::Metric {
        self.v.metric()
    }
}

#[derive(Clone)]
pub struct UntypedVecBuilder {}

impl MetricVecBuilder for UntypedVecBuilder {
    type Output = Untyped;

    fn build(&self, desc: &Desc, vals: &[&str]) -> Result<Untyped> {
        Untyped::with_desc(desc.clone(), vals)
    }
}

/// `UntypedVec` is a Collector that bundles a set of Untypeds that all share the same
/// Desc, but have different values for their variable labels. This is used if
/// you want to count the same thing partitioned by various dimensions.
pub type UntypedVec = MetricVec<UntypedVecBuilder>;

impl UntypedVec {
    /// `new` creates a new `UntypedVec` based on the provided `Opts` and
    /// partitioned by the given label names. At least one label name must be
    /// provided.
    pub fn new(opts: Opts, label_names: &[&str]) -> Result<UntypedVec> {
        let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
        let desc = try!(Desc::new(opts.fq_name(), opts.help, variable_names, opts.const_labels));
        let metric_vec = MetricVec::create(desc, proto::MetricType::UNTYPED, UntypedVecBuilder {});

        Ok(metric_vec as UntypedVec)
    }
}

#[cfg(test)]
mod tests {
    use metrics::{Opts, Collector};

    use super::*;

    #[test]
    fn test_untyped() {
        let opts = Opts::new("test", "test help").const_label("a", "1").const_label("b", "2");
        let untyped = Untyped::with_opts(opts).unwrap();
        untyped.inc();
        assert_eq!(untyped.get() as u64, 1);
        untyped.add(42.0);
        assert_eq!(untyped.get() as u64, 43);
        untyped.sub(42.0);
        assert_eq!(untyped.get() as u64, 1);
        untyped.dec();
        assert_eq!(untyped.get() as u64, 0);
        untyped.set(42.0);
        assert_eq!(untyped.get() as u64, 42);

        let mf = untyped.collect();
        let m = mf.get_metric().as_ref().get(0).unwrap();
        assert_eq!(m.get_label().len(), 2);
        assert_eq!(m.get_untyped().get_value() as u64, 42);
    }
}
