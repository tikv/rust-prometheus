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

use std::convert::From;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use protobuf::RepeatedField;
use proto;
use desc::Desc;
use errors::{Result, Error};
use value::{Value, make_label_pairs};
use vec::{MetricVec, MetricVecBuilder};
use metrics::{Collector, Metric, build_fq_name};

const DEFAULT_BUCKETS: &'static [f64; 11] = &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5,
                                              5.0, 10.0];

// `BUCKET_LABEL` is used for the label that defines the upper bound of a
// bucket of a histogram ("le" -> "less or equal").
const BUCKET_LABEL: &'static str = "le";


/// `HistogramOpts` bundles the options for creating a Histogram metric. It is
/// mandatory to set Name and Help to a non-empty string. All other fields are
/// optional and can safely be left at their zero value.
pub struct HistogramOpts {
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

    // buckets defines the buckets into which observations are counted. Each
    // element in the slice is the upper inclusive bound of a bucket. The
    // values must be sorted in strictly increasing order. There is no need
    // to add a highest bucket with +Inf bound, it will be added
    // implicitly. The default value is DefBuckets.
    pub buckets: Vec<f64>,
}

impl HistogramOpts {
    pub fn new<S: Into<String>>(name: S, help: S) -> HistogramOpts {
        HistogramOpts {
            namespace: "".to_owned(),
            sub_system: "".to_owned(),
            name: name.into(),
            help: help.into(),
            const_labels: HashMap::new(),
            buckets: Vec::from(DEFAULT_BUCKETS as &'static [f64]),
        }
    }

    pub fn namespace<S: Into<String>>(mut self, namesapce: S) -> Self {
        self.namespace = namesapce.into();
        self
    }

    pub fn sub_system<S: Into<String>>(mut self, sub_system: S) -> Self {
        self.sub_system = sub_system.into();
        self
    }

    pub fn const_labels(mut self, labels: HashMap<String, String>) -> Result<Self> {
        for k in labels.keys() {
            if k == BUCKET_LABEL {
                return Err(Error::Msg("`le` is not allowed as label name in histograms"
                    .to_owned()));
            }
        }

        self.const_labels = labels;
        Ok(self)
    }

    pub fn const_label<S: Into<String>>(mut self, name: S, value: S) -> Result<Self> {
        let name = name.into();
        if name == BUCKET_LABEL {
            return Err(Error::Msg("`le` is not allowed as label name in histograms".to_owned()));
        }

        self.const_labels.insert(name, value.into());
        Ok(self)
    }

    pub fn buckets(mut self, buckets: Vec<f64>) -> Result<Self> {
        // TODO: check buckets order.
        self.buckets = buckets;
        Ok(self)
    }

    pub fn fq_name(&self) -> String {
        build_fq_name(&self.namespace, &self.sub_system, &self.name)
    }
}

#[derive(Debug)]
struct HistogramCore {
    sum: f64,
    count: u64,

    upper_bounds: Vec<f64>,
    counts: Vec<u64>,
}

impl HistogramCore {
    fn with_opts(mut opts: HistogramOpts) -> Result<HistogramCore> {
        if opts.buckets.is_empty() {
            opts.buckets = Vec::from(DEFAULT_BUCKETS as &'static [f64]);
        }

        Ok(HistogramCore {
            sum: 0.0,
            count: 0,
            counts: vec![0; opts.buckets.len()],
            upper_bounds: opts.buckets,
        })
    }

    fn observe(&mut self, v: f64) {
        let mut eq = self.upper_bounds.iter().enumerate().filter(|&(_, f)| *f == v);

        if let Some((i, _)) = eq.next() {
            self.count += 1;
            self.counts[i] += 1;
            self.sum += v;
        }
    }

    fn proto_histogram(&self) -> proto::Histogram {
        let mut h = proto::Histogram::new();
        h.set_sample_sum(self.sum);
        h.set_sample_count(self.count);

        let mut count = 0;
        let mut buckets = Vec::with_capacity(self.upper_bounds.len());
        for (i, upper_bound) in self.upper_bounds.iter().enumerate() {
            count += self.counts[i];
            let mut b = proto::Bucket::new();
            b.set_cumulative_count(count);
            b.set_upper_bound(*upper_bound);
            buckets.push(b);
        }
        h.set_bucket(RepeatedField::from_vec(buckets));

        h
    }
}

// A `Histogram` counts individual observations from an event or sample stream in
// configurable buckets. Similar to a summary, it also provides a sum of
// observations and an observation count.
//
// On the Prometheus server, quantiles can be calculated from a Histogram using
// the histogram_quantile function in the query language.
//
// Note that Histograms, in contrast to Summaries, can be aggregated with the
// Prometheus query language (see the documentation for detailed
// procedures). However, Histograms require the user to pre-define suitable
// buckets, and they are in general less accurate. The Observe method of a
// Histogram has a very low performance overhead in comparison with the Observe
// method of a Summary.
//
// To create Histogram instances, use NewHistogram.
#[derive(Clone)]
pub struct Histogram {
    desc: Desc,
    label_pairs: Vec<proto::LabelPair>,

    core: Arc<RwLock<HistogramCore>>,
}

impl Histogram {
    pub fn with_opts(opts: HistogramOpts) -> Result<Histogram> {
        let desc = try!(Desc::new(opts.fq_name(),
                                  opts.help.clone(),
                                  vec![],
                                  opts.const_labels.clone()));

        let pairs = make_label_pairs(&desc, &[]);
        let core = try!(HistogramCore::with_opts(opts));

        Ok(Histogram {
            desc: desc,
            label_pairs: pairs,

            core: Arc::new(RwLock::new(core)),
        })
    }
}

impl Histogram {
    pub fn observe(&self, v: f64) {
        self.core.write().unwrap().observe(v)
    }
}


impl Metric for Histogram {
    fn metric(&self) -> proto::Metric {

        let mut m = proto::Metric::new();

        m.set_label(RepeatedField::from_vec(self.label_pairs.clone()));

        let core = self.core.read().unwrap();
        let h = core.proto_histogram();
        m.set_histogram(h);

        m
    }
}

impl Collector for Histogram {
    fn desc(&self) -> &Desc {
        &self.desc
    }

    fn collect(&self) -> proto::MetricFamily {
        let mut m = proto::MetricFamily::new();
        m.set_name(self.desc.fq_name.clone());
        m.set_help(self.desc.help.clone());
        m.set_field_type(proto::MetricType::HISTOGRAM);
        m.set_metric(RepeatedField::from_vec(vec![self.metric()]));
        m
    }
}

// #[derive(Clone)]
// pub struct CounterVecBuilder {}

// impl MetricVecBuilder for CounterVecBuilder {
//     type Output = Counter;

//     fn build(&self, desc: &Desc, vals: &[&str]) -> Result<Counter> {
//         Counter::with_desc(desc.clone(), vals)
//     }
// }

// /// `CounterVec` is a Collector that bundles a set of Counters that all share the
// /// same Desc, but have different values for their variable labels. This is used
// /// if you want to count the same thing partitioned by various dimensions
// /// (e.g. number of HTTP requests, partitioned by response code and method).
// pub type CounterVec = MetricVec<CounterVecBuilder>;

// impl CounterVec {
//     pub fn new(opts: Opts, label_names: &[&str]) -> Result<CounterVec> {
//         let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
//         let desc = try!(Desc::new(opts.fq_name(), opts.help, variable_names, opts.const_labels));
//         let metric_vec = MetricVec::create(desc, proto::MetricType::COUNTER, CounterVecBuilder {});

//         Ok(metric_vec as CounterVec)
//     }
// }

#[cfg(test)]
mod tests {
    use metrics::Collector;

    use super::*;

    #[test]
    fn test_histogram() {
        let opts = HistogramOpts::new("test", "test help")
            .const_label("a", "1")
            .unwrap()
            .const_label("b", "2")
            .unwrap();
        let histogram = Histogram::with_opts(opts).unwrap();
        histogram.observe(0.5);
        histogram.observe(1.0);

        let mf = histogram.collect();
        let m = mf.get_metric().as_ref().get(0).unwrap();
        assert_eq!(m.get_label().len(), 2);
        let proto_histogram = m.get_histogram();
        assert_eq!(proto_histogram.get_sample_count(), 2);
        assert_eq!(proto_histogram.get_sample_sum(), 1.5);
    }
}
