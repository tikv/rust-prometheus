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

use atomic64::AtomicF64;
use desc::{Desc, Describer};
use errors::{Error, Result};

use proto::{Counter, Gauge, LabelPair, Metric, MetricFamily, MetricType};
use protobuf::RepeatedField;

/// `ValueType` is an enumeration of metric types that represent a simple value
/// for `Counter` and `Gauge`.
pub enum ValueType {
    Counter,
    Gauge,
}

impl ValueType {
    /// `metric_type` returns the corresponding proto metric type.
    pub fn metric_type(&self) -> MetricType {
        match *self {
            ValueType::Counter => MetricType::COUNTER,
            ValueType::Gauge => MetricType::GAUGE,
        }
    }
}

/// `Value` is a generic metric for Counter, Gauge and Untyped.
/// Its effective type is determined by `ValueType`. This is a low-level
/// building block used by the library to back the implementations of
/// `Counter`, `Gauge`, and `Untyped`.
pub struct Value {
    pub desc: Desc,
    pub val: AtomicF64,
    pub val_type: ValueType,
    pub label_pairs: Vec<LabelPair>,
}

impl Value {
    pub fn new<D: Describer>(
        describer: &D,
        value_type: ValueType,
        val: f64,
        label_values: &[&str],
    ) -> Result<Value> {
        let desc = describer.describe()?;
        if desc.variable_labels.len() != label_values.len() {
            return Err(Error::InconsistentCardinality(
                desc.variable_labels.len(),
                label_values.len(),
            ));
        }

        let label_pairs = make_label_pairs(&desc, label_values);

        Ok(Value {
            desc: desc,
            val: AtomicF64::new(val),
            val_type: value_type,
            label_pairs: label_pairs,
        })
    }

    #[inline]
    pub fn get(&self) -> f64 {
        self.val.get()
    }

    #[inline]
    pub fn set(&self, val: f64) {
        self.val.set(val);
    }

    #[inline]
    pub fn inc_by(&self, val: f64) {
        self.val.inc_by(val);
    }

    #[inline]
    pub fn inc(&self) {
        self.inc_by(1.0);
    }

    #[inline]
    pub fn dec(&self) {
        self.inc_by(-1.0);
    }

    #[inline]
    pub fn dec_by(&self, val: f64) {
        self.inc_by(val * -1.0)
    }

    pub fn metric(&self) -> Metric {
        let mut m = Metric::new();
        m.set_label(RepeatedField::from_vec(self.label_pairs.clone()));

        let val = self.get();
        match self.val_type {
            ValueType::Counter => {
                let mut counter = Counter::new();
                counter.set_value(val);
                m.set_counter(counter);
            }
            ValueType::Gauge => {
                let mut gauge = Gauge::new();
                gauge.set_value(val);
                m.set_gauge(gauge);
            }
        }

        m
    }

    pub fn collect(&self) -> MetricFamily {
        let mut m = MetricFamily::new();
        m.set_name(self.desc.fq_name.clone());
        m.set_help(self.desc.help.clone());
        m.set_field_type(self.val_type.metric_type());
        m.set_metric(RepeatedField::from_vec(vec![self.metric()]));
        m
    }
}

pub fn make_label_pairs(desc: &Desc, label_values: &[&str]) -> Vec<LabelPair> {
    let total_len = desc.variable_labels.len() + desc.const_label_pairs.len();
    if total_len == 0 {
        return vec![];
    }

    if desc.variable_labels.is_empty() {
        return desc.const_label_pairs.clone();
    }

    let mut label_pairs = Vec::with_capacity(total_len);
    for (i, n) in desc.variable_labels.iter().enumerate() {
        let mut label_pair = LabelPair::new();
        label_pair.set_name(n.clone());
        label_pair.set_value(label_values[i].to_owned());
        label_pairs.push(label_pair);
    }

    for label_pair in &desc.const_label_pairs {
        label_pairs.push(label_pair.clone());
    }
    label_pairs.sort();
    label_pairs
}
