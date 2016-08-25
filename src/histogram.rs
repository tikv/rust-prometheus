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

use std::sync::{Arc, RwLock};

use protobuf::RepeatedField;

use proto;
use desc::Desc;
use metrics::{Collector, Metric};
use errors::{Result, Error};
use vec::{MetricVec, MetricVecBuilder};

#[derive(Debug)]
struct HistogramCore {
    sum: f64,
    count: u64,

    upper_bounds: Vec<f64>,
    counts: Vec<u64>,

    label_pairs: Vec<proto::LabelPair>,
}

impl HistogramCore {
    pub fn observe(v: f64) {
        unimplemented!();
    }

    fn metric(&self) -> proto::Metric {
        let mut m = proto::Metric::new();

        m.set_label(RepeatedField::from_vec(self.label_pairs.clone()));

        let mut h = proto::Histogram::new();
        h.set_sample_sum(core.sum);
        h.set_sample_count(core.count);

        let mut count = 0;
        let mut buckets = Vec::with_capacity(self.upper_bounds.len());
        for (i, upper_bound) in self.upper_bounds.iter().enumerate() {
            count += self.counts[i];
            let mut b = proto::Bucket::new();
            b.set_cumulative_count(count);
            b.set_upper_bound(upper_bound);
            buckets.append(b);
        }
        h.set_bucket(RepeatedField::from_vec(buckets));

        m.set_histogram(h);

        m
    }
}

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
	pub buckets Vec<f64>,
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

    core: Arc<RwLock<HistogramCore>>,
}

impl Histogram {
    pub new(opts: HistogramOpts) -> Result<Histogram> {
        with_opts(opts)
    }

    pub fn with_opts(opts: HistogramOpts) -> Result<Histogram> {
        let desc = try!(Desc::new(opts.fq_name(), opts.help, vec![], opts.const_labels));
        Histogram::with_desc(desc, &[])
    }

    fn with_desc(desc: Desc, label_values: &[&str]) -> Result<Histogram> {
        unimplemented!()
        // let core = try!();
        // let v = try!(Value::new(desc, ValueType::Counter, 0.0, label_values));
        // Ok(Counter { v: Arc::new(v) })
    }
}

impl Histogram {
    pub fn observe(v: f64) {
        self.write().unwrap().observe(v)
    }
}


impl Metric for Counter {
    fn metric(&self) -> proto::Metric {
        self.core.read().unwrap().metric()
    }
}

impl Collector for Counter {
    fn desc(&self) -> &Desc {
        &self.desc
    }

    fn collect(&self) -> proto::MetricFamily {
        let mut m = MetricFamily::new();
        m.set_name(self.desc.fq_name.clone());
        m.set_help(self.desc.help.clone());
        m.set_field_type(proto::Histogram);
        m.set_metric(RepeatedField::from_vec(vec![self.metric()]));
        m
    }
}

#[derive(Clone)]
pub struct CounterVecBuilder {}

impl MetricVecBuilder for CounterVecBuilder {
    type Output = Counter;

    fn build(&self, desc: &Desc, vals: &[&str]) -> Result<Counter> {
        Counter::with_desc(desc.clone(), vals)
    }
}

/// `CounterVec` is a Collector that bundles a set of Counters that all share the
/// same Desc, but have different values for their variable labels. This is used
/// if you want to count the same thing partitioned by various dimensions
/// (e.g. number of HTTP requests, partitioned by response code and method).
pub type CounterVec = MetricVec<CounterVecBuilder>;

impl CounterVec {
    pub fn new(opts: Opts, label_names: &[&str]) -> Result<CounterVec> {
        let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
        let desc = try!(Desc::new(opts.fq_name(), opts.help, variable_names, opts.const_labels));
        let metric_vec = MetricVec::create(desc, proto::MetricType::COUNTER, CounterVecBuilder {});

        Ok(metric_vec as CounterVec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use metrics::{Opts, Collector};

    #[test]
    fn test_counter() {
        let opts = Opts::new("test", "test help").const_label("a", "1").const_label("b", "2");
        let counter = Counter::with_opts(opts).unwrap();
        counter.inc();
        assert_eq!(counter.get() as u64, 1);
        counter.inc_by(42.0).unwrap();
        assert_eq!(counter.get() as u64, 43);

        let mf = counter.collect();
        let m = mf.get_metric().as_ref().get(0).unwrap();
        assert_eq!(m.get_label().len(), 2);
        assert_eq!(m.get_counter().get_value() as u64, 43);
    }
}
