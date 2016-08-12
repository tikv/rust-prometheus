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

use std::collections::{HashMap, HashSet};
use std::hash::Hasher;

use fnv::FnvHasher;

use proto::LabelPair;
use errors::{Result, Error};
use metrics::SEPARATOR_BYTE;

// Desc is the descriptor used by every Prometheus Metric. It is essentially
// the immutable meta-data of a Metric. The normal Metric implementations
// included in this package manage their Desc under the hood. Users only have to
// deal with Desc if they use advanced features like the ExpvarCollector or
// custom Collectors and Metrics.
//
// Descriptors registered with the same registry have to fulfill certain
// consistency and uniqueness criteria if they share the same fully-qualified
// name: They must have the same help string and the same label names (aka label
// dimensions) in each, constLabels and variableLabels, but they must differ in
// the values of the constLabels.
//
// Descriptors that share the same fully-qualified names and the same label
// values of their constLabels are considered equal.
//
// Use NewDesc to create new Desc instances.
#[derive(Clone, Debug)]
pub struct Desc {
    // fq_name has been built from Namespace, Subsystem, and Name.
    pub fq_name: String,
    // help provides some helpful information about this metric.
    pub help: String,
    // const_label_pairs contains precalculated DTO label pairs based on
    // the constant labels.
    pub const_label_pairs: Vec<LabelPair>,
    // variable_labels contains names of labels for which the metric
    // maintains variable values.
    pub variable_labels: Vec<String>,
    // id is a hash of the values of the ConstLabels and fqName. This
    // must be unique among all registered descriptors and can therefore be
    // used as an identifier of the descriptor.
    pub id: u64,
    // dim_hash is a hash of the label names (preset and variable) and the
    // Help string. Each Desc with the same fqName must have the same
    // dimHash.
    pub dim_hash: u64,
}

impl Desc {
    pub fn new(fq_name: String,
               help: String,
               variable_labels: Vec<String>,
               const_labels: HashMap<String, String>)
               -> Result<Desc> {
        let mut desc = Desc {
            fq_name: fq_name.clone(),
            help: help,
            const_label_pairs: Vec::with_capacity(const_labels.len()),
            variable_labels: vec![],
            id: 0,
            dim_hash: 0,
        };

        if desc.help.is_empty() {
            return Err(Error::Msg("empty help string".into()));
        }

        // TODO: check valid metric fq_name.

        let mut label_values = Vec::with_capacity(const_labels.len() + 1);
        label_values.push(fq_name);
        let mut label_names = Vec::with_capacity(const_labels.len() + variable_labels.len());
        let mut label_name_set = HashSet::new();

        for label_name in const_labels.keys() {
            // TODO: check invalid label name
            label_names.push(label_name.clone());
            label_name_set.insert(label_name.clone());
        }

        label_names.sort();

        // ... so that we can now add const label values in the order of their names.
        for label_name in &label_names {
            label_values.push(const_labels.get(label_name).cloned().unwrap());
        }

        // Now add the variable label names, but prefix them with something that
        // cannot be in a regular label name. That prevents matching the label
        // dimension with a different mix between preset and variable labels.
        for label_name in variable_labels {
            // TODO: check invalid label name
            label_names.push(format!("${}", label_name));
            label_name_set.insert(label_name);
        }

        if label_names.len() != label_name_set.len() {
            return Err(Error::Msg("duplicate label names".into()));
        }

        let mut vh = FnvHasher::default();
        for val in &label_values {
            vh.write(val.as_bytes());
            vh.write_u8(SEPARATOR_BYTE);
        }

        desc.id = vh.finish();

        // Sort labelNames so that order doesn't matter for the hash.
        label_names.sort();
        // Now hash together (in this order) the help string and the sorted
        // label names.
        let mut lh = FnvHasher::default();
        lh.write(desc.help.as_bytes());
        lh.write_u8(SEPARATOR_BYTE);
        for label_name in &label_names {
            lh.write(label_name.as_bytes());
            lh.write_u8(SEPARATOR_BYTE);
        }
        desc.dim_hash = lh.finish();

        for (key, value) in const_labels {
            let mut label_pair = LabelPair::new();
            label_pair.set_name(key);
            label_pair.set_value(value);
            desc.const_label_pairs.push(label_pair);
        }

        desc.const_label_pairs.sort();

        Ok(desc)
    }
}
