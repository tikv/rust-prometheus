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

use desc::Desc;
use proto::{self, LabelPair};
use std::cmp::{Ord, Ordering, Eq, PartialOrd};

pub const SEPARATOR_BYTE: u8 = 0xFF;

pub trait Metric: Sync + Send {
    /// `desc` returns the descriptor for the Metric
    fn desc(&self) -> &Desc;
    /// `metric` encodes the Metric into a "Metric" Protocol Buffer data
    /// transmission object.
    fn metric(&self) -> proto::Metric;
}

/// `Opts` bundles the options for creating most Metric types.
pub struct Opts {
    // namespace, sub_system, and name are components of the fully-qualified
    // name of the Metric (created by joining these components with
    // "_"). Only Name is mandatory, the others merely help structuring the
    // name. Note that the fully-qualified name of the metric must be a
    // valid Prometheus metric name.
    pub namespace: String,
    pub sub_system: String,
    pub name: String,

    // help provides information about this metric. Mandatory!
    //
    // Metrics with the same fully-qualified name must have the same Help
    // string.
    pub help: String,

    // const_labels are used to attach fixed labels to this metric. Metrics
    // with the same fully-qualified name must have the same label names in
    // their ConstLabels.
    //
    // Note that in most cases, labels have a value that varies during the
    // lifetime of a process. Those labels are usually managed with a metric
    // vector collector (like CounterVec, GaugeVec, UntypedVec). ConstLabels
    // serve only special purposes. One is for the special case where the
    // value of a label does not change during the lifetime of a process,
    // e.g. if the revision of the running binary is put into a
    // label. Another, more advanced purpose is if more than one Collector
    // needs to collect Metrics with the same fully-qualified name. In that
    // case, those Metrics must differ in the values of their
    // ConstLabels. See the Collector examples.
    //
    // If the value of a label never changes (not even between binaries),
    // that label most likely should not be a label at all (but part of the
    // metric name).
    pub const_labels: HashMap<String, String>,
}

impl Opts {
    pub fn new<S: Into<String>>(namespace: S, sub_system: S, name: S, help: S) -> Opts {
        Opts::with_label(namespace, sub_system, name, help, HashMap::new())
    }

    pub fn with_label<S: Into<String>>(namespace: S,
                                       sub_system: S,
                                       name: S,
                                       help: S,
                                       const_labels: HashMap<String, String>)
                                       -> Opts {
        Opts {
            namespace: namespace.into(),
            sub_system: sub_system.into(),
            name: name.into(),
            help: help.into(),
            const_labels: const_labels,
        }
    }

    pub fn fq_name(&self) -> String {
        build_fq_name(&self.namespace, &self.sub_system, &self.name)
    }
}

impl Ord for LabelPair {
    fn cmp(&self, other: &LabelPair) -> Ordering {
        if self.get_name() < other.get_name() {
            return Ordering::Less;
        } else if self.get_name() > other.get_name() {
            return Ordering::Greater;
        }

        Ordering::Equal
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
/// (with NewDesc) directly.
pub fn build_fq_name(namespace: &str, sub_system: &str, name: &str) -> String {
    if name.is_empty() {
        return "".to_owned();
    }

    if !namespace.is_empty() && !sub_system.is_empty() {
        return format!("{}_{}_{}", namespace, sub_system, name);
    } else if !namespace.is_empty() {
        return format!("{}_{}", namespace, name);
    } else if !sub_system.is_empty() {
        return format!("{}_{}", sub_system, name);
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

        for (namespace, sub_system, name, res) in tbl {
            assert_eq!(&build_fq_name(namespace, sub_system, name), res);
        }
    }
}
