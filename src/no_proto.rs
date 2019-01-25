//! Non-generated version of required structures provided by the protobuf.
//! This version is used when the `protobuf` feature is turned off.

#![allow(missing_docs)]

use std::default::Default;

lazy_static! {
    static ref DEFAULT_GAUGE: Gauge = Gauge::default();
    static ref DEFAULT_HISTOGRAM: Histogram = Histogram::default();
    static ref DEFAULT_SUMMARY: Summary = Summary::default();
    static ref DEFAULT_COUNTER: Counter = Counter::default();
    static ref DEFAULT_UNTYPED: Untyped = Untyped::default();
}

#[macro_export]
macro_rules! from_vec {
    ($e: expr) => {
        $e
    };
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct LabelPair {
    name: String,
    value: String,
}

impl LabelPair {
    pub fn new() -> LabelPair {
        Default::default()
    }

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: String) {
        self.name = v;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: String) {
        self.value = v;
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Gauge {
    value: f64,
}

impl Gauge {
    pub fn new() -> Gauge {
        Default::default()
    }

    // optional double value = 1;

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Counter {
    // message fields
    value: f64,
}

impl Counter {
    pub fn new() -> Counter {
        Default::default()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Quantile {
    quantile: f64,
    value: f64,
}

impl Quantile {
    pub fn new() -> Quantile {
        Default::default()
    }

    pub fn set_quantile(&mut self, v: f64) {
        self.quantile = v;
    }

    pub fn get_quantile(&self) -> f64 {
        self.quantile
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Summary {
    sample_count: u64,
    sample_sum: f64,
    quantile: Vec<Quantile>,
}

impl Summary {
    pub fn new() -> Summary {
        Default::default()
    }

    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = v;
    }

    pub fn get_sample_count(&self) -> u64 {
        self.sample_count
    }

    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = v;
    }

    pub fn get_sample_sum(&self) -> f64 {
        self.sample_sum
    }

    pub fn set_quantile(&mut self, v: Vec<Quantile>) {
        self.quantile = v;
    }

    pub fn get_quantile(&self) -> &[Quantile] {
        &self.quantile
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Untyped {
    value: f64,
}

impl Untyped {
    pub fn new() -> Untyped {
        Default::default()
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Histogram {
    sample_count: u64,
    sample_sum: f64,
    bucket: Vec<Bucket>,
}

impl Histogram {
    pub fn new() -> Histogram {
        Default::default()
    }

    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = v;
    }

    pub fn get_sample_count(&self) -> u64 {
        self.sample_count
    }

    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = v;
    }

    pub fn get_sample_sum(&self) -> f64 {
        self.sample_sum
    }

    pub fn set_bucket(&mut self, v: Vec<Bucket>) {
        self.bucket = v;
    }

    pub fn get_bucket(&self) -> &[Bucket] {
        &self.bucket
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Bucket {
    // message fields
    cumulative_count: u64,
    upper_bound: f64,
}

impl Bucket {
    pub fn new() -> Bucket {
        Default::default()
    }

    pub fn set_cumulative_count(&mut self, v: u64) {
        self.cumulative_count = v;
    }

    pub fn get_cumulative_count(&self) -> u64 {
        self.cumulative_count
    }

    pub fn set_upper_bound(&mut self, v: f64) {
        self.upper_bound = v;
    }

    pub fn get_upper_bound(&self) -> f64 {
        self.upper_bound
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Metric {
    // message fields
    label: Vec<LabelPair>,
    gauge: Option<Gauge>,
    counter: Option<Counter>,
    summary: Option<Summary>,
    untyped: Option<Untyped>,
    histogram: Option<Histogram>,
    timestamp_ms: i64,
}

impl Metric {
    pub fn new() -> Metric {
        Default::default()
    }

    pub fn set_label(&mut self, v: Vec<LabelPair>) {
        self.label = v;
    }

    pub fn get_label(&self) -> &[LabelPair] {
        &self.label
    }

    pub fn clear_gauge(&mut self) {
        self.gauge = None
    }

    pub fn has_gauge(&self) -> bool {
        self.gauge.is_some()
    }

    pub fn set_gauge(&mut self, v: Gauge) {
        self.gauge = Some(v);
    }

    pub fn get_gauge(&self) -> &Gauge {
        self.gauge.as_ref().unwrap_or_else(|| &DEFAULT_GAUGE)
    }

    pub fn clear_counter(&mut self) {
        self.counter = None
    }

    pub fn has_counter(&self) -> bool {
        self.counter.is_some()
    }

    pub fn set_counter(&mut self, v: Counter) {
        self.counter = Some(v);
    }

    pub fn get_counter(&self) -> &Counter {
        self.counter.as_ref().unwrap_or_else(|| &DEFAULT_COUNTER)
    }

    pub fn clear_summary(&mut self) {
        self.summary = None
    }

    pub fn has_summary(&self) -> bool {
        self.summary.is_some()
    }

    pub fn set_summary(&mut self, v: Summary) {
        self.summary = Some(v);
    }

    pub fn get_summary(&self) -> &Summary {
        self.summary.as_ref().unwrap_or_else(|| &DEFAULT_SUMMARY)
    }

    pub fn clear_untyped(&mut self) {
        self.untyped = None
    }

    pub fn has_untyped(&self) -> bool {
        self.untyped.is_some()
    }

    pub fn set_untyped(&mut self, v: Untyped) {
        self.untyped = Some(v);
    }

    pub fn get_untyped(&self) -> &Untyped {
        self.untyped.as_ref().unwrap_or_else(|| &DEFAULT_UNTYPED)
    }

    pub fn clear_histogram(&mut self) {
        self.histogram = None
    }

    pub fn has_histogram(&self) -> bool {
        self.histogram.is_some()
    }

    pub fn set_histogram(&mut self, v: Histogram) {
        self.histogram = Some(v);
    }

    pub fn get_histogram(&self) -> &Histogram {
        self.histogram
            .as_ref()
            .unwrap_or_else(|| &DEFAULT_HISTOGRAM)
    }

    pub fn set_timestamp_ms(&mut self, v: i64) {
        self.timestamp_ms = v;
    }

    pub fn get_timestamp_ms(&self) -> i64 {
        self.timestamp_ms
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy)]
pub enum MetricType {
    COUNTER = 0,
    GAUGE = 1,
    SUMMARY = 2,
    UNTYPED = 3,
    HISTOGRAM = 4,
}

impl Default for MetricType {
    fn default() -> Self {
        MetricType::COUNTER
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct MetricFamily {
    name: String,
    help: String,
    field_type: MetricType,
    metric: Vec<Metric>,
}

impl MetricFamily {
    pub fn new() -> MetricFamily {
        Default::default()
    }

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn set_name(&mut self, v: String) {
        self.name = v;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Param is passed by value, moved
    pub fn set_help(&mut self, v: String) {
        self.help = v;
    }

    pub fn get_help(&self) -> &str {
        &self.help
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MetricType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> MetricType {
        self.field_type
    }

    pub fn clear_metric(&mut self) {
        self.metric.clear();
    }

    pub fn set_metric(&mut self, v: Vec<Metric>) {
        self.metric = v;
    }

    pub fn mut_metric(&mut self) -> &mut Vec<Metric> {
        &mut self.metric
    }

    pub fn take_metric(&mut self) -> Vec<Metric> {
        ::std::mem::replace(&mut self.metric, Vec::new())
    }

    pub fn get_metric(&self) -> &[Metric] {
        &self.metric
    }
}
