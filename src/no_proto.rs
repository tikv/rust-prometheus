//! Non-generated version of required structures provided by the protobuf.
//! This version is used when the protobuf feature is is turned off.

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

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name = None
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut String {
        if self.name.is_none() {
            self.name = Some(Default::default());
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> String {
        self.name.take().unwrap_or_else(|| String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value = None
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut String {
        if self.value.is_none() {
            self.value = Some(Default::default());
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> String {
        self.value.take().unwrap_or_else(|| String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
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

    pub fn clear_value(&mut self) {
        self.value = None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn get_value(&self) -> f64 {
        self.value.unwrap_or(0.)
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

    // optional double value = 1;

    pub fn clear_value(&mut self) {
        self.value = None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn get_value(&self) -> f64 {
        self.value.unwrap_or(0.)
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Quantile {
    // message fields
    quantile: f64,
    value: f64,
}

impl Quantile {
    pub fn new() -> Quantile {
        Default::default()
    }

    // optional double quantile = 1;

    pub fn clear_quantile(&mut self) {
        self.quantile = None;
    }

    pub fn has_quantile(&self) -> bool {
        self.quantile.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quantile(&mut self, v: f64) {
        self.quantile = v;
    }

    pub fn get_quantile(&self) -> f64 {
        self.quantile.unwrap_or(0.)
    }

    // optional double value = 2;

    pub fn clear_value(&mut self) {
        self.value = None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn get_value(&self) -> f64 {
        self.value.unwrap_or(0.)
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Summary {
    // message fields
    sample_count: u64,
    sample_sum: f64,
    quantile: Vec<Quantile>,
}

impl Summary {
    pub fn new() -> Summary {
        Default::default()
    }

    // optional uint64 sample_count = 1;

    pub fn clear_sample_count(&mut self) {
        self.sample_count = None;
    }

    pub fn has_sample_count(&self) -> bool {
        self.sample_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = v;
    }

    pub fn get_sample_count(&self) -> u64 {
        self.sample_count.unwrap_or(0)
    }

    // optional double sample_sum = 2;

    pub fn clear_sample_sum(&mut self) {
        self.sample_sum = None;
    }

    pub fn has_sample_sum(&self) -> bool {
        self.sample_sum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = v;
    }

    pub fn get_sample_sum(&self) -> f64 {
        self.sample_sum.unwrap_or(0.)
    }

    // repeated .io.prometheus.client.Quantile quantile = 3;

    pub fn clear_quantile(&mut self) {
        self.quantile.clear();
    }

    // Param is passed by value, moved
    pub fn set_quantile(&mut self, v: Vec<Quantile>) {
        self.quantile = v;
    }

    // Mutable pointer to the field.
    pub fn mut_quantile(&mut self) -> &mut Vec<Quantile> {
        &mut self.quantile
    }

    // Take field
    pub fn take_quantile(&mut self) -> Vec<Quantile> {
        ::std::mem::replace(&mut self.quantile, Vec::new())
    }

    pub fn get_quantile(&self) -> &[Quantile] {
        &self.quantile
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Untyped {
    // message fields
    value: f64,
}

impl Untyped {
    pub fn new() -> Untyped {
        Default::default()
    }

    // optional double value = 1;

    pub fn clear_value(&mut self) {
        self.value = None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn get_value(&self) -> f64 {
        self.value.unwrap_or(0.)
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Histogram {
    // message fields
    sample_count: u64,
    sample_sum: f64,
    bucket: Vec<Bucket>,
}

impl Histogram {
    pub fn new() -> Histogram {
        Default::default()
    }

    // optional uint64 sample_count = 1;

    pub fn clear_sample_count(&mut self) {
        self.sample_count = None;
    }

    pub fn has_sample_count(&self) -> bool {
        self.sample_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = v;
    }

    pub fn get_sample_count(&self) -> u64 {
        self.sample_count.unwrap_or(0)
    }

    // optional double sample_sum = 2;

    pub fn clear_sample_sum(&mut self) {
        self.sample_sum = None;
    }

    pub fn has_sample_sum(&self) -> bool {
        self.sample_sum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = v;
    }

    pub fn get_sample_sum(&self) -> f64 {
        self.sample_sum.unwrap_or(0.)
    }

    // repeated .io.prometheus.client.Bucket bucket = 3;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: Vec<Bucket>) {
        self.bucket = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bucket(&mut self) -> &mut Vec<Bucket> {
        &mut self.bucket
    }

    // Take field
    pub fn take_bucket(&mut self) -> Vec<Bucket> {
        ::std::mem::replace(&mut self.bucket, Vec::new())
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

    // optional uint64 cumulative_count = 1;

    pub fn clear_cumulative_count(&mut self) {
        self.cumulative_count = None;
    }

    pub fn has_cumulative_count(&self) -> bool {
        self.cumulative_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cumulative_count(&mut self, v: u64) {
        self.cumulative_count = v;
    }

    pub fn get_cumulative_count(&self) -> u64 {
        self.cumulative_count.unwrap_or(0)
    }

    // optional double upper_bound = 2;

    pub fn clear_upper_bound(&mut self) {
        self.upper_bound = None;
    }

    pub fn has_upper_bound(&self) -> bool {
        self.upper_bound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upper_bound(&mut self, v: f64) {
        self.upper_bound = v;
    }

    pub fn get_upper_bound(&self) -> f64 {
        self.upper_bound.unwrap_or(0.)
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

    // repeated .io.prometheus.client.LabelPair label = 1;

    pub fn clear_label(&mut self) {
        self.label.clear();
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: Vec<LabelPair>) {
        self.label = v;
    }

    // Mutable pointer to the field.
    pub fn mut_label(&mut self) -> &mut Vec<LabelPair> {
        &mut self.label
    }

    // Take field
    pub fn take_label(&mut self) -> Vec<LabelPair> {
        ::std::mem::replace(&mut self.label, Vec::new())
    }

    pub fn get_label(&self) -> &[LabelPair] {
        &self.label
    }

    // optional .io.prometheus.client.Gauge gauge = 2;

    pub fn clear_gauge(&mut self) {
        self.gauge = None
    }

    pub fn has_gauge(&self) -> bool {
        self.gauge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gauge(&mut self, v: Gauge) {
        self.gauge = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gauge(&mut self) -> &mut Gauge {
        if self.gauge.is_none() {
            self.gauge = Some(Default::default());
        }
        self.gauge.as_mut().unwrap()
    }

    // Take field
    pub fn take_gauge(&mut self) -> Gauge {
        self.gauge.take().unwrap_or_else(|| Gauge::new())
    }

    pub fn get_gauge(&self) -> &Gauge {
        self.gauge.as_ref().unwrap_or_else(|| &DEFAULT_GAUGE)
    }

    // optional .io.prometheus.client.Counter counter = 3;

    pub fn clear_counter(&mut self) {
        self.counter = None
    }

    pub fn has_counter(&self) -> bool {
        self.counter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_counter(&mut self, v: Counter) {
        self.counter = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_counter(&mut self) -> &mut Counter {
        if self.counter.is_none() {
            self.counter = Some(Default::default());
        }
        self.counter.as_mut().unwrap()
    }

    // Take field
    pub fn take_counter(&mut self) -> Counter {
        self.counter.take().unwrap_or_else(|| Counter::new())
    }

    pub fn get_counter(&self) -> &Counter {
        self.counter.as_ref().unwrap_or_else(|| &DEFAULT_COUNTER)
    }

    // optional .io.prometheus.client.Summary summary = 4;

    pub fn clear_summary(&mut self) {
        self.summary = None
    }

    pub fn has_summary(&self) -> bool {
        self.summary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_summary(&mut self, v: Summary) {
        self.summary = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_summary(&mut self) -> &mut Summary {
        if self.summary.is_none() {
            self.summary = Some(Default::default());
        }
        self.summary.as_mut().unwrap()
    }

    // Take field
    pub fn take_summary(&mut self) -> Summary {
        self.summary.take().unwrap_or_else(|| Summary::new())
    }

    pub fn get_summary(&self) -> &Summary {
        self.summary.as_ref().unwrap_or_else(|| &DEFAULT_SUMMARY)
    }

    // optional .io.prometheus.client.Untyped untyped = 5;

    pub fn clear_untyped(&mut self) {
        self.untyped = None
    }

    pub fn has_untyped(&self) -> bool {
        self.untyped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_untyped(&mut self, v: Untyped) {
        self.untyped = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_untyped(&mut self) -> &mut Untyped {
        if self.untyped.is_none() {
            self.untyped = Some(Default::default());
        }
        self.untyped.as_mut().unwrap()
    }

    // Take field
    pub fn take_untyped(&mut self) -> Untyped {
        self.untyped.take().unwrap_or_else(|| Untyped::new())
    }

    pub fn get_untyped(&self) -> &Untyped {
        self.untyped.as_ref().unwrap_or_else(|| &DEFAULT_UNTYPED)
    }

    // optional .io.prometheus.client.Histogram histogram = 7;

    pub fn clear_histogram(&mut self) {
        self.histogram = None
    }

    pub fn has_histogram(&self) -> bool {
        self.histogram.is_some()
    }

    // Param is passed by value, moved
    pub fn set_histogram(&mut self, v: Histogram) {
        self.histogram = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_histogram(&mut self) -> &mut Histogram {
        if self.histogram.is_none() {
            self.histogram = Some(Default::default());
        }
        self.histogram.as_mut().unwrap()
    }

    // Take field
    pub fn take_histogram(&mut self) -> Histogram {
        self.histogram.take().unwrap_or_else(|| Histogram::new())
    }

    pub fn get_histogram(&self) -> &Histogram {
        self.histogram
            .as_ref()
            .unwrap_or_else(|| &DEFAULT_HISTOGRAM)
    }

    // optional int64 timestamp_ms = 6;

    pub fn clear_timestamp_ms(&mut self) {
        self.timestamp_ms = None;
    }

    pub fn has_timestamp_ms(&self) -> bool {
        self.timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp_ms(&mut self, v: i64) {
        self.timestamp_ms = v;
    }

    pub fn get_timestamp_ms(&self) -> i64 {
        self.timestamp_ms.unwrap_or(0)
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

#[derive(PartialEq, Clone, Default, Debug)]
pub struct MetricFamily {
    // message fields
    name: String,
    help: String,
    field_type: MetricType,
    metric: Vec<Metric>,
}

impl MetricFamily {
    pub fn new() -> MetricFamily {
        Default::default()
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name = None
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut String {
        if self.name.is_none() {
            self.name = Some(Default::default());
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> String {
        self.name.take().unwrap_or_else(|| String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string help = 2;

    pub fn clear_help(&mut self) {
        self.help = None
    }

    pub fn has_help(&self) -> bool {
        self.help.is_some()
    }

    // Param is passed by value, moved
    pub fn set_help(&mut self, v: String) {
        self.help = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_help(&mut self) -> &mut String {
        if self.help.is_none() {
            self.help = Some(Default::default());
        }
        self.help.as_mut().unwrap()
    }

    // Take field
    pub fn take_help(&mut self) -> String {
        self.help.take().unwrap_or_else(|| String::new())
    }

    pub fn get_help(&self) -> &str {
        match self.help.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .io.prometheus.client.MetricType type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type = None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MetricType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> MetricType {
        self.field_type.unwrap_or(MetricType::COUNTER)
    }

    // repeated .io.prometheus.client.Metric metric = 4;

    pub fn clear_metric(&mut self) {
        self.metric.clear();
    }

    // Param is passed by value, moved
    pub fn set_metric(&mut self, v: Vec<Metric>) {
        self.metric = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metric(&mut self) -> &mut Vec<Metric> {
        &mut self.metric
    }

    // Take field
    pub fn take_metric(&mut self) -> Vec<Metric> {
        ::std::mem::replace(&mut self.metric, Vec::new())
    }

    pub fn get_metric(&self) -> &[Metric] {
        &self.metric
    }
}
