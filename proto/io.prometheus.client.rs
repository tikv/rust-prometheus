#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelPair {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gauge {
    #[prost(double, optional, tag="1")]
    pub value: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Counter {
    #[prost(double, optional, tag="1")]
    pub value: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quantile {
    #[prost(double, optional, tag="1")]
    pub quantile: ::core::option::Option<f64>,
    #[prost(double, optional, tag="2")]
    pub value: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Summary {
    #[prost(uint64, optional, tag="1")]
    pub sample_count: ::core::option::Option<u64>,
    #[prost(double, optional, tag="2")]
    pub sample_sum: ::core::option::Option<f64>,
    #[prost(message, repeated, tag="3")]
    pub quantile: ::prost::alloc::vec::Vec<Quantile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Untyped {
    #[prost(double, optional, tag="1")]
    pub value: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Histogram {
    #[prost(uint64, optional, tag="1")]
    pub sample_count: ::core::option::Option<u64>,
    #[prost(double, optional, tag="2")]
    pub sample_sum: ::core::option::Option<f64>,
    /// Ordered in increasing order of upper_bound, +Inf bucket is optional.
    #[prost(message, repeated, tag="3")]
    pub bucket: ::prost::alloc::vec::Vec<Bucket>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    /// Cumulative in increasing order.
    #[prost(uint64, optional, tag="1")]
    pub cumulative_count: ::core::option::Option<u64>,
    /// Inclusive.
    #[prost(double, optional, tag="2")]
    pub upper_bound: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    #[prost(message, repeated, tag="1")]
    pub label: ::prost::alloc::vec::Vec<LabelPair>,
    #[prost(message, optional, tag="2")]
    pub gauge: ::core::option::Option<Gauge>,
    #[prost(message, optional, tag="3")]
    pub counter: ::core::option::Option<Counter>,
    #[prost(message, optional, tag="4")]
    pub summary: ::core::option::Option<Summary>,
    #[prost(message, optional, tag="5")]
    pub untyped: ::core::option::Option<Untyped>,
    #[prost(message, optional, tag="7")]
    pub histogram: ::core::option::Option<Histogram>,
    #[prost(int64, optional, tag="6")]
    pub timestamp_ms: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricFamily {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub help: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="MetricType", optional, tag="3")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="4")]
    pub metric: ::prost::alloc::vec::Vec<Metric>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetricType {
    Counter = 0,
    Gauge = 1,
    Summary = 2,
    Untyped = 3,
    Histogram = 4,
}
