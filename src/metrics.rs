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

use std::collections::HashMap;
use std::cmp::{Ord, Ordering, Eq, PartialOrd};

use desc::{Desc, Describer};
use errors::Result;
use proto::{self, LabelPair};

pub const SEPARATOR_BYTE: u8 = 0xFF;

/// `Collector` is the trait that can be used to collect metrics.
/// A Collector has to be registered for collection.
pub trait Collector: Sync + Send {
    /// `desc` returns descriptors for metrics.
    fn desc(&self) -> Vec<&Desc>;

    /// `collect` collects metrics.
    fn collect(&self) -> Vec<proto::MetricFamily>;
}

/// `Metric` is the trait that models a single sample value with its meta data being
/// exported to Prometheus.
pub trait Metric: Sync + Send + Clone {
    /// `metric` returns the protocol Metric.
    fn metric(&self) -> proto::Metric;
}

/// `Opts` bundles the options for creating most Metric types.
#[derive(Debug, Clone)]
pub struct Opts {
    /// namespace, subsystem, and name are components of the fully-qualified
    /// name of the Metric (created by joining these components with
    /// "_"). Only Name is mandatory, the others merely help structuring the
    /// name. Note that the fully-qualified name of the metric must be a
    /// valid Prometheus metric name.
    pub namespace: String,
    pub subsystem: String,
    pub name: String,

    /// help provides information about this metric. Mandatory!
    ///
    /// Metrics with the same fully-qualified name must have the same Help
    /// string.
    pub help: String,

    /// const_labels are used to attach fixed labels to this metric. Metrics
    /// with the same fully-qualified name must have the same label names in
    /// their ConstLabels.
    ///
    /// Note that in most cases, labels have a value that varies during the
    /// lifetime of a process. Those labels are usually managed with a metric
    /// vector collector (like CounterVec, GaugeVec, UntypedVec). ConstLabels
    /// serve only special purposes. One is for the special case where the
    /// value of a label does not change during the lifetime of a process,
    /// e.g. if the revision of the running binary is put into a
    /// label. Another, more advanced purpose is if more than one Collector
    /// needs to collect Metrics with the same fully-qualified name. In that
    /// case, those Metrics must differ in the values of their
    /// ConstLabels. See the Collector examples.
    ///
    /// If the value of a label never changes (not even between binaries),
    /// that label most likely should not be a label at all (but part of the
    /// metric name).
    pub const_labels: HashMap<String, String>,

    /// variable_labels contains names of labels for which the metric maintains
    /// variable values. Metrics with the same fully-qualified name must have
    /// the same label names in their variable_labels.
    ///
    /// Note that variable_labels is used in `MetricVec`. To create a single
    /// metric must leave it empty.
    pub variable_labels: Vec<String>,
}

impl Opts {
    /// `new` creates the Opts with the `name` and `help` arguments.
    pub fn new(name: &str, help: &str) -> Opts {
        Opts {
            namespace: String::new(),
            subsystem: String::new(),
            name: name.to_string(),
            help: help.to_string(),
            const_labels: HashMap::new(),
            variable_labels: Vec::new(),
        }
    }

    /// `namespace` sets the namespace.
    pub fn namespace(mut self, namesapce: &str) -> Self {
        self.namespace = namesapce.to_string();
        self
    }

    /// `subsystem` sets the sub system.
    pub fn subsystem(mut self, subsystem: &str) -> Self {
        self.subsystem = subsystem.to_string();
        self
    }

    /// `const_labels` sets the const labels.
    pub fn const_labels(mut self, const_labels: HashMap<String, String>) -> Self {
        self.const_labels = const_labels;
        self
    }

    /// `const_label` adds a const label.
    pub fn const_label(mut self, name: &str, value: &str) -> Self {
        self.const_labels.insert(name.to_string(), value.to_string());
        self
    }

    /// `variable_labels` sets the variable labels.
    pub fn variable_labels(mut self, variable_labels: Vec<String>) -> Self {
        self.variable_labels = variable_labels;
        self
    }

    /// `variable_label` adds a variable label.
    pub fn variable_label(mut self, name: &str) -> Self {
        self.variable_labels.push(name.to_string());
        self
    }

    /// `fq_name` returns the fq_name.
    pub fn fq_name(&self) -> String {
        build_fq_name(&self.namespace, &self.subsystem, &self.name)
    }
}

impl Describer for Opts {
    fn describe(&self) -> Result<Desc> {
        Desc::new(self.fq_name(),
                  self.help.clone(),
                  self.variable_labels.clone(),
                  self.const_labels.clone())
    }
}

impl Ord for LabelPair {
    fn cmp(&self, other: &LabelPair) -> Ordering {
        self.get_name().cmp(other.get_name())
    }
}

impl Eq for LabelPair {}

impl PartialOrd for LabelPair {
    fn partial_cmp(&self, other: &LabelPair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// `build_fq_name` joins the given three name components by "_". Empty name
/// components are ignored. If the name parameter itself is empty, an empty
/// string is returned, no matter what. Metric implementations included in this
/// library use this function internally to generate the fully-qualified metric
/// name from the name component in their Opts. Users of the library will only
/// need this function if they implement their own Metric or instantiate a Desc
/// directly.
pub fn build_fq_name(namespace: &str, subsystem: &str, name: &str) -> String {
    if name.is_empty() {
        return "".to_owned();
    }

    if !namespace.is_empty() && !subsystem.is_empty() {
        return format!("{}_{}_{}", namespace, subsystem, name);
    } else if !namespace.is_empty() {
        return format!("{}_{}", namespace, name);
    } else if !subsystem.is_empty() {
        return format!("{}_{}", subsystem, name);
    }

    name.to_owned()
}

#[cfg(test)]
mod tests {
    use std::cmp::{Ord, Ordering};
    use super::*;
    use proto::LabelPair;

    fn new_label_pair(name: &str, value: &str) -> LabelPair {
        let mut l = LabelPair::new();
        l.set_name(name.to_owned());
        l.set_value(value.to_owned());
        l
    }

    #[test]
    fn test_label_cmp() {
        let tbl = vec![
            ("k1", "k2", Ordering::Less),
            ("k1", "k1", Ordering::Equal),
            ("k1", "k0", Ordering::Greater),
        ];

        for (l1, l2, order) in tbl {
            let lhs = new_label_pair(l1, l1);
            let rhs = new_label_pair(l2, l2);
            assert_eq!(lhs.cmp(&rhs), order);
        }
    }

    #[test]
    fn test_build_fq_name() {
        let tbl = vec![
        ("a", "b", "c", "a_b_c"),
        ("", "b", "c", "b_c"),
        ("a", "", "c", "a_c"),
        ("", "", "c", "c"),
        ("a", "b", "", ""),
        ("a", "", "", ""),
        ("", "b", "", ""),
        (" ", "", "", ""),
        ];

        for (namespace, subsystem, name, res) in tbl {
            assert_eq!(&build_fq_name(namespace, subsystem, name), res);
        }
    }
}
