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
use value::make_label_pairs;
use vec::{MetricVec, MetricVecBuilder};
use metrics::{Collector, Metric, Opts};

pub const DEFAULT_BUCKETS: &'static [f64; 11] = &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0,
                                                  2.5, 5.0, 10.0];

// `BUCKET_LABEL` is used for the label that defines the upper bound of a
// bucket of a histogram ("le" -> "less or equal").
pub const BUCKET_LABEL: &'static str = "le";

#[inline]
fn check_bucket_lable(label: &str) -> Result<()> {
    if label == BUCKET_LABEL {
        return Err(Error::Msg("`le` is not allowed as label name in histograms".to_owned()));
    }

    Ok(())
}

/// `HistogramOpts` bundles the options for creating a Histogram metric. It is
/// mandatory to set Name and Help to a non-empty string. All other fields are
/// optional and can safely be left at their zero value.
pub struct HistogramOpts {
    pub common_opts: Opts,

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
            common_opts: Opts::new(name, help),
            buckets: Vec::from(DEFAULT_BUCKETS as &'static [f64]),
        }
    }

    pub fn namespace<S: Into<String>>(mut self, namesapce: S) -> Self {
        self.common_opts.namespace = namesapce.into();
        self
    }

    pub fn sub_system<S: Into<String>>(mut self, sub_system: S) -> Self {
        self.common_opts.sub_system = sub_system.into();
        self
    }

    pub fn const_labels(mut self, labels: HashMap<String, String>) -> Self {
        self.common_opts = self.common_opts.const_labels(labels);
        self
    }

    pub fn const_label<S: Into<String>>(mut self, name: S, value: S) -> Self {
        self.common_opts = self.common_opts.const_label(name, value);
        self
    }

    pub fn fq_name(&self) -> String {
        self.common_opts.fq_name()
    }

    pub fn buckets(mut self, buckets: Vec<f64>) -> Self {
        self.buckets = buckets;
        self
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
    fn new() -> HistogramCore {
        HistogramCore::with_buckets(vec![]).unwrap()
    }

    fn with_buckets(mut buckets: Vec<f64>) -> Result<HistogramCore> {
        if buckets.is_empty() {
            buckets = Vec::from(DEFAULT_BUCKETS as &'static [f64]);
        } else {
            let mut tail_inf = false;
            for (i, upper_bound) in buckets.iter().enumerate() {
                if i < (buckets.len() - 1) {
                    if *upper_bound >= buckets[i + 1] {
                        return Err(Error::Msg(format!("histogram buckets must be in increasing \
                                                       order: {} >= {}",
                                                      upper_bound,
                                                      buckets[i + 1])));
                    } else if upper_bound.is_sign_positive() && upper_bound.is_infinite() {
                        tail_inf = true;
                    }
                }
            }
            if tail_inf {
                // The +Inf bucket is implicit. Remove it here.
                buckets.pop();
            }
        }

        Ok(HistogramCore {
            sum: 0.0,
            count: 0,
            counts: vec![0; buckets.len()],
            upper_bounds: buckets,
        })
    }

    fn observe(&mut self, v: f64) {
        // Try find the bucket.
        let mut iter = self.upper_bounds.iter().enumerate().filter(|&(_, f)| v <= *f);
        if let Some((i, _)) = iter.next() {
            self.counts[i] += 1;
        }

        self.count += 1;
        self.sum += v;
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
                                  opts.common_opts.help.clone(),
                                  vec![],
                                  opts.common_opts.const_labels.clone()));

        Histogram::with_desc(desc, &[])
    }

    fn with_desc(desc: Desc, label_values: &[&str]) -> Result<Histogram> {
        for name in &desc.variable_labels {
            try!(check_bucket_lable(&name));
        }
        for pair in &desc.const_label_pairs {
            try!(check_bucket_lable(pair.get_name()));
        }

        let pairs = make_label_pairs(&desc, label_values);
        let core = HistogramCore::new();

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

#[derive(Clone)]
pub struct HistogramVecBuilder {}

impl MetricVecBuilder for HistogramVecBuilder {
    type Output = Histogram;

    fn build(&self, desc: &Desc, vals: &[&str]) -> Result<Histogram> {
        Histogram::with_desc(desc.clone(), vals)
    }
}

// `HistogramVec` is a Collector that bundles a set of Histograms that all share the
// same Desc, but have different values for their variable labels. This is used
// if you want to count the same thing partitioned by various dimensions
// (e.g. HTTP request latencies, partitioned by status code and method). Create
// instances with NewHistogramVec.
pub type HistogramVec = MetricVec<HistogramVecBuilder>;

impl HistogramVec {
    pub fn new(opts: HistogramOpts, label_names: &[&str]) -> Result<HistogramVec> {
        let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
        let desc = try!(Desc::new(opts.fq_name(),
                                  opts.common_opts.help,
                                  variable_names,
                                  opts.common_opts.const_labels));
        let metric_vec =
            MetricVec::create(desc, proto::MetricType::HISTOGRAM, HistogramVecBuilder {});

        Ok(metric_vec as HistogramVec)
    }
}

/// `linear_buckets` creates 'count' buckets, each 'width' wide, where the lowest
/// bucket has an upper bound of 'start'. The final +Inf bucket is not counted
/// and not included in the returned slice. The returned slice is meant to be
/// used for the Buckets field of `HistogramOpts`.
///
/// The function returns an error if 'count' is zero or negative.
pub fn linear_buckets(start: f64, width: f64, count: usize) -> Result<Vec<f64>> {
    if count < 1 {
        return Err(Error::Msg("LinearBuckets needs a positive count".to_owned()));
    }

    let mut next = start;
    let mut buckets = Vec::with_capacity(count);
    for _ in 0..count {
        buckets.push(next);
        next += width;
    }

    Ok(buckets)
}

/// `exponential_buckets` creates 'count' buckets, where the lowest bucket has an
/// upper bound of 'start' and each following bucket's upper bound is 'factor'
/// times the previous bucket's upper bound. The final +Inf bucket is not counted
/// and not included in the returned slice. The returned slice is meant to be
/// used for the Buckets field of `HistogramOpts`.
///
/// The function returns an error if 'count' is 0 or negative, if 'start' is 0
/// or negative, or if 'factor' is less than or equal 1.
pub fn exponential_buckets(start: f64, factor: f64, count: usize) -> Result<Vec<f64>> {
    if count < 1 {
        return Err(Error::Msg("exponential_buckets needs a positive count".to_owned()));
    }
    if start <= 0.0 {
        return Err(Error::Msg("exponential_buckets needs a positive start value".to_owned()));
    }
    if factor <= 1.0 {
        return Err(Error::Msg("exponential_buckets needs a factor greater than 1".to_owned()));
    }

    let mut next = start;
    let mut buckets = Vec::with_capacity(count);
    for _ in 0..count {
        buckets.push(next);
        next *= factor;
    }

    Ok(buckets)
}

#[cfg(test)]
mod tests {
    use metrics::Collector;
    use std::f64::EPSILON;

    use super::*;

    #[test]
    fn test_histogram() {
        let opts = HistogramOpts::new("test", "test help")
            .const_label("a", "1")
            .const_label("b", "2");
        let histogram = Histogram::with_opts(opts).unwrap();
        histogram.observe(0.5);
        histogram.observe(1.0);

        let mf = histogram.collect();
        let m = mf.get_metric().as_ref().get(0).unwrap();
        assert_eq!(m.get_label().len(), 2);
        let proto_histogram = m.get_histogram();
        assert_eq!(proto_histogram.get_sample_count(), 2);
        assert!((proto_histogram.get_sample_sum() - 1.5).abs() < EPSILON);
    }
}
