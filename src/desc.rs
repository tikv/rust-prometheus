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

use std::collections::{HashMap, BTreeSet};
use std::hash::Hasher;

use fnv::FnvHasher;
use regex::Regex;

use proto::LabelPair;
use errors::{Result, Error};
use metrics::SEPARATOR_BYTE;

lazy_static! {
    static ref VALID_METRIC_NAME: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_:]*$").unwrap();
    static ref VALID_LABEL_NAME: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

/// Desc is the descriptor used by every Prometheus Metric. It is essentially
/// the immutable meta-data of a Metric. The normal Metric implementations
/// included in this package manage their Desc under the hood.
///
/// Descriptors registered with the same registry have to fulfill certain
/// consistency and uniqueness criteria if they share the same fully-qualified
/// name: They must have the same help string and the same label names (aka label
/// dimensions) in each, constLabels and variableLabels, but they must differ in
/// the values of the constLabels.
///
/// Descriptors that share the same fully-qualified names and the same label
/// values of their constLabels are considered equal.
#[derive(Clone, Debug)]
pub struct Desc {
    /// fq_name has been built from Namespace, Subsystem, and Name.
    pub fq_name: String,
    /// help provides some helpful information about this metric.
    pub help: String,
    /// const_label_pairs contains precalculated DTO label pairs based on
    /// the constant labels.
    pub const_label_pairs: Vec<LabelPair>,
    /// variable_labels contains names of labels for which the metric
    /// maintains variable values.
    pub variable_labels: Vec<String>,
    /// id is a hash of the values of the ConstLabels and fqName. This
    /// must be unique among all registered descriptors and can therefore be
    /// used as an identifier of the descriptor.
    pub id: u64,
    /// dim_hash is a hash of the label names (preset and variable) and the
    /// Help string. Each Desc with the same fqName must have the same
    /// dimHash.
    pub dim_hash: u64,
}

impl Desc {
    /// Initializes a new Desc. Errors are recorded in the Desc
    /// and will be reported on registration time. variableLabels and constLabels can
    /// be nil if no such labels should be set. fqName and help must not be empty.
    pub fn new(fq_name: String,
               help: String,
               variable_labels: Vec<String>,
               const_labels: HashMap<String, String>)
               -> Result<Desc> {
        let mut desc = Desc {
            fq_name: fq_name.clone(),
            help: help,
            const_label_pairs: Vec::with_capacity(const_labels.len()),
            variable_labels: variable_labels,
            id: 0,
            dim_hash: 0,
        };

        if desc.help.is_empty() {
            return Err(Error::Msg("empty help string".into()));
        }

        if !VALID_METRIC_NAME.is_match(&desc.fq_name) {
            return Err(Error::Msg(format!("'{}' is not a valid metric name", desc.fq_name)));
        }

        let mut label_values = Vec::with_capacity(const_labels.len() + 1);
        label_values.push(fq_name);

        let mut label_names = BTreeSet::new();

        for label_name in const_labels.keys() {
            if !VALID_LABEL_NAME.is_match(&label_name) {
                return Err(Error::Msg(format!("'{}' is not a valid label name", &label_name)));
            }

            if !label_names.insert(label_name.clone()) {
                return Err(Error::Msg(format!("duplicate const label name {}", label_name)));
            }
        }

        // ... so that we can now add const label values in the order of their names.
        for label_name in &label_names {
            label_values.push(const_labels.get(label_name).cloned().unwrap());
        }

        // Now add the variable label names, but prefix them with something that
        // cannot be in a regular label name. That prevents matching the label
        // dimension with a different mix between preset and variable labels.
        for label_name in &desc.variable_labels {
            if !VALID_LABEL_NAME.is_match(label_name) {
                return Err(Error::Msg(format!("'{}' is not a valid label name", &label_name)));
            }

            if !label_names.insert(format!("${}", label_name)) {
                return Err(Error::Msg(format!("duplicate variable label name {}", label_name)));
            }
        }

        let mut vh = FnvHasher::default();
        for val in &label_values {
            vh.write(val.as_bytes());
            vh.write_u8(SEPARATOR_BYTE);
        }

        desc.id = vh.finish();

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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use desc::Desc;
    use errors::Error;

    #[test]
    fn test_invalid_const_label_name() {
        for &name in &["-dash", "9gag", ":colon", "colon:", "has space"] {
            let res = Desc::new("name".into(),
                                "help".into(),
                                vec![name.into()],
                                HashMap::new())
                .err()
                .expect(format!("expected error for {}", name).as_ref());
            match res {
                Error::Msg(msg) => assert_eq!(msg, format!("'{}' is not a valid label name", name)),
                other => panic!(other),
            };
        }
    }

    #[test]
    fn test_invalid_variable_label_name() {
        for &name in &["-dash", "9gag", ":colon", "colon:", "has space"] {
            let mut labels = HashMap::new();
            labels.insert(name.into(), "value".into());
            let res = Desc::new("name".into(), "help".into(), vec![], labels)
                .err()
                .expect(format!("expected error for {}", name).as_ref());
            match res {
                Error::Msg(msg) => assert_eq!(msg, format!("'{}' is not a valid label name", name)),
                other => panic!(other),
            };
        }
    }

    #[test]
    fn test_invalid_metric_name() {
        for &name in &["-dash", "9gag", ":colon", "has space"] {
            let res = Desc::new(name.into(), "help".into(), vec![], HashMap::new())
                .err()
                .expect(format!("expected error for {}", name).as_ref());
            match res {
                Error::Msg(msg) => {
                    assert_eq!(msg, format!("'{}' is not a valid metric name", name))
                }
                other => panic!(other),
            };
        }
    }
}
