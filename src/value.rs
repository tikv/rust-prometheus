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

use std::sync::RwLock;

use protobuf::RepeatedField;

use proto::{LabelPair, Metric, Counter, Gauge, Untyped};
use desc::Desc;
use errors::{Result, Error};

pub enum ValueType {
    Counter,
    Gauge,
    Untyped,
}

/// `Value` is a generic metric for Counter, Gauge and Untyped.
pub struct Value {
    pub desc: Desc,
    // TODO: like prometheus client go, use atomic u64.
    pub val: RwLock<f64>,
    pub val_type: ValueType,
    pub label_pairs: Vec<LabelPair>,
}

impl Value {
    pub fn new(desc: Desc,
               value_type: ValueType,
               val: f64,
               label_values: Vec<String>)
               -> Result<Value> {
        if desc.variable_labels.len() != label_values.len() {
            return Err(Error::InconsistentCardinality);
        }

        let label_pairs = make_label_pairs(&desc, label_values);

        Ok(Value {
            desc: desc,
            val: RwLock::new(val),
            val_type: value_type,
            label_pairs: label_pairs,
        })
    }

    #[inline]
    pub fn set(&self, val: f64) {
        *self.val.write().unwrap() = val;
    }

    #[inline]
    pub fn get(&self) -> f64 {
        *self.val.read().unwrap()
    }

    #[inline]
    pub fn inc(&self) {
        self.add(1.0);
    }

    #[inline]
    pub fn dec(&self) {
        self.add(-1.0);
    }

    #[inline]
    pub fn add(&self, val: f64) {
        *self.val.write().unwrap() += val;
    }

    #[inline]
    pub fn sub(&self, val: f64) {
        self.add(val * -1.0)
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
            ValueType::Untyped => {
                let mut untyped = Untyped::new();
                untyped.set_value(val);
                m.set_untyped(untyped);
            }
        }

        m
    }
}

pub fn make_label_pairs(desc: &Desc, label_values: Vec<String>) -> Vec<LabelPair> {
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
        label_pair.set_value(label_values[i].clone());
        label_pairs.push(label_pair);
    }

    for label_pair in &desc.const_label_pairs {
        label_pairs.push(label_pair.clone());
    }
    label_pairs.sort();
    label_pairs
}
