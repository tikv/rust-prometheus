// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/// Creates a HashMap of labels with specified name-value pairs for Prometheus metrics.
///
/// This macro simplifies the creation of label dictionaries used in Prometheus metrics,
/// making it easier to define constant labels for metric options or label values for
/// metric vectors.
///
/// # Syntax
///
/// ```
/// // Create a HashMap with label key-value pairs:
/// labels!{
///     "key1" => "value1",
///     "key2" => "value2",
///     // ...
/// }
///
/// // Create an empty label HashMap:
/// labels!{}
/// ```
///
/// # Return Value
///
/// Returns a `HashMap<K, V>` where `K` and `V` are the types of the keys and values provided.
/// Typically, this will be `HashMap<&str, &str>` or `HashMap<String, String>` depending on
/// the inputs.
///
/// # Examples
///
/// Basic label creation:
///
/// ```rust
/// use prometheus::labels;
///
/// // Create a set of labels
/// let api_labels = labels!{
///     "endpoint" => "/users",
///     "method" => "GET"
/// };
///
/// // Use with a counter vector
/// // counter_vec.with_label_values(&["/users", "GET"]).inc();
/// // Or equivalently:
/// // counter_vec.with(&api_labels).inc();
/// ```
///
/// For constant labels in metric options:
///
/// ```rust
/// use prometheus::{labels, opts};
///
/// // Create constant labels for a service
/// let service_labels = labels!{
///     "service" => "authentication",
///     "environment" => "production",
///     "tier" => "backend"
/// };
///
/// // Use in metric options
/// let counter_opts = opts!(
///     "auth_failures_total",
///     "Total number of authentication failures",
///     service_labels
/// );
/// ```
///
/// Empty label set:
///
/// ```rust
/// use prometheus::labels;
/// use std::collections::HashMap;
///
/// // Create an empty label set
/// let empty_labels: HashMap<&str, &str> = labels!{};
/// assert!(empty_labels.is_empty());
/// ```
///
/// With dynamic values:
///
/// ```rust
/// use prometheus::labels;
///
/// let app_name = "payment-service";
/// let app_version = "v2.3.1";
///
/// let app_labels = labels!{
///     "application" => app_name,
///     "version" => app_version
/// };
/// ```
/// # Related Macros
///
/// - [`opts!`][crate::opts]: Creates options for counter and gauge metrics
/// - [`histogram_opts!`][crate::histogram_opts]: Creates options for histogram metrics
/// - [`register_counter_vec!`][crate::register_counter_vec]: Registers a counter vector with the default registry
/// - [`register_gauge_vec!`][crate::register_gauge_vec]: Registers a gauge vector with the default registry
#[macro_export]
macro_rules! labels {
    ( $( $ KEY : expr => $ VALUE : expr ),* $(,)? ) => {
        {
            use std::collections::HashMap;

            let mut lbs = HashMap::new();
            $(
                lbs.insert($KEY, $VALUE);
            )*

            lbs
        }
    };
}

#[test]
fn test_labels_without_trailing_comma() {
    let labels = labels! {
        "test" => "hello",
        "foo" => "bar"
    };
    assert_eq!(labels.len(), 2);
    assert!(labels.contains_key("test"));
    assert_eq!(*(labels.get("test").unwrap()), "hello");
}

/// Creates an [`Opts`][crate::Opts] configuration for counter and gauge metrics.
///
/// `Opts` defines the metadata for basic Prometheus metrics, including the name, help text,
/// and optional constant labels. This macro simplifies the creation of options for counters,
/// gauges, and other basic metrics.
///
/// # Syntax
///
/// This macro can be used in multiple ways:
///
/// ```
/// // Basic configuration with name and help:
/// opts!(name, help);
///
/// // With constant labels:
/// opts!(name, help, const_labels);
///
/// // With multiple label sets that will be merged:
/// opts!(name, help, labels1, labels2, ...);
/// ```
///
/// # Parameters
///
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `const_labels`: One or more HashMaps of string key-value pairs defined with the [`labels!`][crate::labels] macro,
///   providing constant labels that will be included with every observation.
///
/// # Return Value
///
/// Returns an [`Opts`][crate::Opts] struct that can be used to create counter, gauge, and other metric types.
///
/// # Examples
///
/// Basic configuration:
///
/// ```rust
/// use prometheus::opts;
///
/// // Create basic options for a counter
/// let opts = opts!(
///     "http_requests_total",
///     "Total number of HTTP requests"
/// );
/// ```
///
/// With constant labels:
///
/// ```rust
/// use prometheus::{opts, labels};
///
/// // Create options with constant labels
/// let opts = opts!(
///     "http_requests_total",
///     "Total number of HTTP requests",
///     labels!{
///         "service" => "api",
///         "environment" => "production"
///     }
/// );
/// ```
///
/// With multiple sets of labels:
///
/// ```rust
/// use prometheus::{opts, labels};
///
/// // Define common labels for all services
/// let common_labels = labels!{
///     "environment" => "production",
///     "datacenter" => "us-west"
/// };
///
/// // Define service-specific labels
/// let service_labels = labels!{
///     "service" => "authentication",
///     "tier" => "frontend"
/// };
///
/// // Create options with merged labels
/// let opts = opts!(
///     "active_sessions",
///     "Number of active user sessions",
///     common_labels,
///     service_labels
/// );
/// ```
///
/// # Related Macros
///
/// - [`histogram_opts!`][crate::histogram_opts]: Creates options for histogram metrics
/// - [`labels!`][crate::labels]: Creates label HashMaps for constant labels
/// - [`register_counter!`][crate::register_counter]: Registers a counter with the default registry
/// - [`register_gauge!`][crate::register_gauge]: Registers a gauge with the default registry
#[macro_export]
macro_rules! opts {
    ( $ NAME : expr , $ HELP : expr $ ( , $ CONST_LABELS : expr ) * $ ( , ) ? ) => {
        {
            use std::collections::HashMap;

            let opts = $crate::Opts::new($NAME, $HELP);
            let lbs = HashMap::<String, String>::new();
            $(
                #[allow(clippy::redundant_locals)]
                let mut lbs = lbs;
                lbs.extend($CONST_LABELS.iter().map(|(k, v)| ((*k).into(), (*v).into())));
            )*

            opts.const_labels(lbs)
        }
    }
}

#[test]
fn test_opts_trailing_comma() {
    let name = "test_opts";
    let help = "test opts help";

    let opts = opts!(name, help,);
    assert_eq!(opts.name, name);
    assert_eq!(opts.help, help);

    let opts = opts!(name, help, labels! {"test" => "hello", "foo" => "bar",},);
    assert_eq!(opts.const_labels.len(), 2);
    assert!(opts.const_labels.contains_key("foo"));
    assert_eq!(opts.const_labels.get("foo").unwrap(), "bar");

    let opts = opts!(
        name,
        help,
        labels! {"test" => "hello", "foo" => "bar",},
        labels! {"ans" => "42",},
    );
    assert_eq!(opts.const_labels.len(), 3);
    assert!(opts.const_labels.contains_key("ans"));
    assert_eq!(opts.const_labels.get("ans").unwrap(), "42");
}

/// Creates a [`HistogramOpts`][crate::HistogramOpts] configuration for histogram metrics.
///
/// `HistogramOpts` defines the metadata and bucket configuration for histogram metrics.
/// Histograms sample observations and count them in configurable buckets, which is essential
/// for measuring distributions of values like request durations or response sizes.
///
/// # Syntax
///
/// This macro can be used in three ways:
///
/// ```
/// // Basic configuration with name and help:
/// histogram_opts!(name, help);
///
/// // With custom buckets:
/// histogram_opts!(name, help, buckets);
///
/// // With custom buckets and constant labels:
/// histogram_opts!(name, help, buckets, const_labels);
/// ```
///
/// # Parameters
///
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `buckets`: A vector of `f64` values (`Vec<f64>`) or a bucket generator function defining the bucket boundaries.
/// - `const_labels`: A HashMap of string key-value pairs defined with the [`labels!`][crate::labels] macro, providing constant labels.
///
/// # Return Value
///
/// Returns a [`HistogramOpts`][crate::HistogramOpts] struct that can be used to create a histogram metric.
///
/// # Examples
///
/// Basic configuration:
///
/// ```rust
/// use prometheus::histogram_opts;
///
/// // Create basic histogram options
/// let opts = histogram_opts!(
///     "http_request_duration_seconds",
///     "HTTP request duration in seconds"
/// );
/// ```
///
/// With custom buckets:
///
/// ```rust
/// use prometheus::{histogram_opts, linear_buckets};
///
/// // Create histogram options with linear buckets for latency measurements
/// let opts = histogram_opts!(
///     "http_request_duration_seconds",
///     "HTTP request duration in seconds",
///     linear_buckets(0.01, 0.05, 10).unwrap() // 10 buckets from 0.01s to 0.46s
/// );
///
/// // Or with explicit buckets
/// let opts = histogram_opts!(
///     "http_request_size_bytes",
///     "HTTP request size in bytes",
///     vec![100.0, 1000.0, 10000.0, 100000.0, 1000000.0]
/// );
/// ```
///
/// With custom buckets and constant labels:
///
/// ```rust
/// use prometheus::{histogram_opts, labels};
///
/// // Create histogram options with custom buckets and constant labels
/// let opts = histogram_opts!(
///     "api_request_duration_seconds",
///     "API request duration in seconds by version",
///     vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0],
///     labels!{
///         "service" => "user_api",
///         "version" => "v2"
///     }
/// );
/// ```
///
/// Using exponential buckets:
///
/// ```rust
/// use prometheus::{histogram_opts, exponential_buckets};
///
/// // Create histogram options with exponential buckets for wide-range measurements
/// let opts = histogram_opts!(
///     "file_size_bytes",
///     "Processed file size in bytes",
///     exponential_buckets(1024.0, 4.0, 8).unwrap() // 8 buckets: 1KB, 4KB, 16KB, 64KB, 256KB, 1MB, 4MB, 16MB
/// );
/// ```
///
/// # Bucket Configuration
///
/// Proper bucket configuration is crucial for histogram accuracy:
///
/// - Too few buckets will not provide enough resolution
/// - Too many buckets will increase memory usage
/// - Bucket upper bounds should cover the expected range of values
/// - Consider using helper functions like [`linear_buckets`][crate::linear_buckets] or
///   [`exponential_buckets`][crate::exponential_buckets] for systematic bucket creation
///
/// # Related Macros
///
/// - [`opts!`][crate::opts]: Creates basic metric options for counters and gauges
/// - [`labels!`][crate::labels]: Creates label HashMaps for constant labels
/// - [`register_histogram!`][crate::register_histogram]: Registers a histogram with the default registry
/// - [`register_histogram_vec!`][crate::register_histogram_vec]: Registers a histogram vector with the default registry
#[macro_export(local_inner_macros)]
macro_rules! histogram_opts {
    ($NAME:expr, $HELP:expr $(,)?) => {{
        $crate::HistogramOpts::new($NAME, $HELP)
    }};

    ($NAME:expr, $HELP:expr, $BUCKETS:expr $(,)?) => {{
        let hopts = histogram_opts!($NAME, $HELP);
        hopts.buckets($BUCKETS)
    }};

    ($NAME:expr, $HELP:expr, $BUCKETS:expr, $CONST_LABELS:expr $(,)?) => {{
        let hopts = histogram_opts!($NAME, $HELP, $BUCKETS);
        hopts.const_labels($CONST_LABELS)
    }};
}

#[test]
fn test_histogram_opts_trailing_comma() {
    use crate::linear_buckets;

    let name = "test_histogram_opts";
    let help = "test opts help";

    let opts = histogram_opts!(name, help,);
    assert_eq!(opts.common_opts.name, name);
    assert_eq!(opts.common_opts.help, help);

    let opts = histogram_opts!(name, help, linear_buckets(1.0, 0.5, 4).unwrap(),);
    assert_eq!(opts.common_opts.name, name);
    assert_eq!(opts.common_opts.help, help);
    assert_eq!(opts.buckets.len(), 4);

    let opts = histogram_opts!(
        name,
        help,
        vec![1.0, 2.0],
        labels! {"key".to_string() => "value".to_string(),},
    );
    assert_eq!(opts.common_opts.name, name);
    assert_eq!(opts.common_opts.help, help);
    assert_eq!(opts.buckets.len(), 2);
    assert!(opts.common_opts.const_labels.contains_key("key"));
    assert_eq!(opts.common_opts.const_labels.get("key").unwrap(), "value");
}

/// Creates a [`Counter`][crate::Counter] and registers it to the default registry.
///
/// A counter is a cumulative metric that represents a single monotonically increasing counter
/// whose value can only increase or be reset to zero on restart. Counters are typically used
/// to count requests, errors, tasks completed, or other discrete events in an application.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_counter!(opts);
///
/// // Using name and help strings directly:
/// register_counter!(name, help);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the counter's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
///
/// # Return Value
///
/// Returns a `Result<Counter, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered.
///
/// # Examples
///
/// Using with pre-defined options:
///
/// ```rust
/// use prometheus::{register_counter, opts};
///
/// // Create and register the counter
/// let opts = opts!("http_requests_total", "Total number of HTTP requests processed");
/// let request_counter = register_counter!(opts).unwrap();
///
/// // Increment the counter by 1
/// request_counter.inc();
///
/// // Increment the counter by a specific amount
/// request_counter.inc_by(2.5);
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::register_counter;
///
/// // Create and register the counter directly
/// let bytes_sent = register_counter!(
///     "network_bytes_sent_total",
///     "Total number of bytes sent over the network"
/// ).unwrap();
///
/// // Increment after sending data
/// bytes_sent.inc_by(1024.0);
/// ```
///
/// # Related Macros
///
/// - [`register_counter_with_registry!`][crate::register_counter_with_registry]: Registers a counter with a custom registry
/// - [`register_int_counter!`][crate::register_int_counter]: Registers an integer-only counter with the default registry
/// - [`register_counter_vec!`][crate::register_counter_vec]: Registers a counter vector with the default registry
/// - [`counter!`][crate::counter]: Creates a counter without registering it
///
/// For custom registry usage, see [`register_counter_with_registry!`][crate::register_counter_with_registry].
#[macro_export(local_inner_macros)]
macro_rules! register_counter {
    (@of_type $TYPE:ident, $OPTS:expr) => {{
        let counter = $crate::$TYPE::with_opts($OPTS).unwrap();
        $crate::register(Box::new(counter.clone())).map(|()| counter)
    }};

    ($OPTS:expr $(,)?) => {{
        register_counter!(@of_type Counter, $OPTS)
    }};

    ($NAME:expr, $HELP:expr $(,)?) => {{
        register_counter!(opts!($NAME, $HELP))
    }};
}

#[test]
fn test_register_counter_trailing_comma() {
    let opts = opts!("test_macro_counter_1", "help",);
    let res1 = register_counter!(opts,);
    assert!(res1.is_ok());

    let res2 = register_counter!("test_macro_counter_2", "help",);
    assert!(res2.is_ok());
}

/// Creates a [`Counter`][crate::Counter] and registers it with a custom registry.
///
/// This macro allows for registering counters to custom registries rather than
/// the default global registry. A counter is a cumulative metric that represents a
/// single monotonically increasing value that can only go up (or be reset to zero
/// when the process restarts).
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_counter_with_registry!(opts, registry);
///
/// // Using name and help strings directly:
/// register_counter_with_registry!(name, help, registry);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the counter's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `registry`: The [`Registry`][crate::Registry] instance where the counter will be registered.
///
/// # Return Value
///
/// Returns a `Result<Counter, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered in the specified registry.
///
/// # Examples
///
/// Using with a custom registry and pre-defined options:
///
/// ```rust
/// use prometheus::{register_counter_with_registry, opts, Registry};
/// use std::collections::HashMap;
///
/// // Create a custom registry with a prefix and labels
/// let mut labels = HashMap::new();
/// labels.insert("environment".to_string(), "production".to_string());
/// labels.insert("service".to_string(), "api".to_string());
/// let custom_registry = Registry::new_custom(
///     Some("mycompany".to_string()),
///     Some(labels)
/// ).unwrap();
///
/// // Create and register the counter
/// let opts = opts!("http_requests_total", "Total number of HTTP requests processed");
/// let request_counter = register_counter_with_registry!(opts, custom_registry).unwrap();
///
/// // Increment the counter
/// request_counter.inc();
/// request_counter.inc_by(3.5);
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::{register_counter_with_registry, Registry};
///
/// // Create a simple custom registry
/// let metrics_registry = Registry::new();
///
/// // Create and register the counter directly
/// let data_processed = register_counter_with_registry!(
///     "data_processed_bytes_total",
///     "Total number of bytes processed",
///     metrics_registry
/// ).unwrap();
///
/// // Increment the counter
/// data_processed.inc_by(4096.0);
/// ```
///
/// # Related Macros
///
/// - [`register_counter!`][crate::register_counter]: Registers a counter with the default registry
/// - [`register_int_counter_with_registry!`][crate::register_int_counter_with_registry]: Registers an integer-only counter with a custom registry
/// - [`register_counter_vec_with_registry!`][crate::register_counter_vec_with_registry]: Registers a counter vector with a custom registry
/// - [`counter!`][crate::counter]: Creates a counter without registering it
#[macro_export(local_inner_macros)]
macro_rules! register_counter_with_registry {
    (@of_type $TYPE: ident, $OPTS:expr, $REGISTRY:expr) => {{
        let counter = $crate::$TYPE::with_opts($OPTS).unwrap();
        $REGISTRY.register(Box::new(counter.clone())).map(|()| counter)
    }};

    ($OPTS:expr, $REGISTRY:expr $(,)?) => {{
        register_counter_with_registry!(@of_type Counter, $OPTS, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $REGISTRY:expr $(,)?) => {{
        register_counter_with_registry!(opts!($NAME, $HELP), $REGISTRY)
    }};
}

#[test]
fn test_register_counter_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();
    let opts = opts!("test_macro_counter_1", "help",);
    let res1 = register_counter_with_registry!(opts, custom_registry,);
    assert!(res1.is_ok());

    let res2 = register_counter_with_registry!("test_macro_counter_2", "help", custom_registry,);
    assert!(res2.is_ok());
}

/// Creates an [`IntCounter`][crate::IntCounter] and registers it to the default registry.
///
/// An [`IntCounter`][crate::IntCounter] is a specialized version of [`Counter`][crate::Counter] that only
/// accepts integer increments. It's optimized for performance when you don't need floating-point precision.
/// Like regular counters, an IntCounter can only go up (or be reset to zero when the process restarts).
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_int_counter!(opts);
///
/// // Using name and help strings directly:
/// register_int_counter!(name, help);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the counter's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
///
/// # Return Value
///
/// Returns a `Result<IntCounter, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered.
///
/// # Examples
///
/// Using with pre-defined options:
///
/// ```rust
/// use prometheus::{register_int_counter, opts};
///
/// // Create and register the int counter
/// let opts = opts!("http_requests_total", "Total number of HTTP requests processed");
/// let request_counter = register_int_counter!(opts).unwrap();
///
/// // Increment the counter by 1
/// request_counter.inc();
///
/// // Increment the counter by a specific amount
/// request_counter.inc_by(42);
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::register_int_counter;
///
/// // Create and register the int counter directly
/// let errors = register_int_counter!(
///     "database_errors_total",
///     "Total number of database errors encountered"
/// ).unwrap();
///
/// // Increment counters
/// errors.inc();
/// ```
///
/// # Related Macros
///
/// - [`register_counter!`][crate::register_counter]: Registers a floating-point counter with the default registry
/// - [`register_int_counter_with_registry!`][crate::register_int_counter_with_registry]: Registers an integer counter with a custom registry
/// - [`register_int_counter_vec!`][crate::register_int_counter_vec]: Registers an integer counter vector with the default registry
/// - [`int_counter!`][crate::int_counter]: Creates an integer counter without registering it
///
/// For custom registry usage, see [`register_int_counter_with_registry!`][crate::register_int_counter_with_registry].
#[macro_export(local_inner_macros)]
macro_rules! register_int_counter {
    ($OPTS:expr $(,)?) => {{
        register_counter!(@of_type IntCounter, $OPTS)
    }};

    ($NAME:expr, $HELP:expr $(,)?) => {{
        register_int_counter!(opts!($NAME, $HELP))
    }};
}

/// Creates an [`IntCounter`][crate::IntCounter] and registers it with a custom registry.
///
/// This macro allows for registering integer counters to custom registries rather than
/// the default global registry. An [`IntCounter`][crate::IntCounter] is a specialized version
/// of [`Counter`][crate::Counter] that only accepts integer increments and is optimized
/// for performance when counting discrete events.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_int_counter_with_registry!(opts, registry);
///
/// // Using name and help strings directly:
/// register_int_counter_with_registry!(name, help, registry);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the counter's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `registry`: The [`Registry`][crate::Registry] instance where the counter will be registered.
///
/// # Return Value
///
/// Returns a `Result<IntCounter, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered in the specified registry.
///
/// # Examples
///
/// Using with a custom registry and pre-defined options:
///
/// ```rust
/// use prometheus::{register_int_counter_with_registry, opts, Registry};
/// use std::collections::HashMap;
///
/// // Create a custom registry with a prefix and labels
/// let mut labels = HashMap::new();
/// labels.insert("component".to_string(), "api_server".to_string());
/// let custom_registry = Registry::new_custom(
///     Some("app".to_string()),
///     Some(labels)
/// ).unwrap();
///
/// // Create and register the int counter
/// let opts = opts!("requests_total", "Total number of requests processed");
/// let request_counter = register_int_counter_with_registry!(opts, custom_registry).unwrap();
///
/// // Increment the counter
/// request_counter.inc();
/// request_counter.inc_by(5);
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::{register_int_counter_with_registry, Registry};
///
/// // Create a simple custom registry
/// let metrics_registry = Registry::new();
///
/// // Create and register the int counter directly
/// let job_failures = register_int_counter_with_registry!(
///     "job_failures_total",
///     "Total number of failed jobs",
///     metrics_registry
/// ).unwrap();
///
/// // Increment counter when a job fails
/// job_failures.inc();
/// ```
///
/// # Related Macros
///
/// - [`register_int_counter!`][crate::register_int_counter]: Registers an integer counter with the default registry
/// - [`register_counter_with_registry!`][crate::register_counter_with_registry]: Registers a floating-point counter with a custom registry
/// - [`register_int_counter_vec_with_registry!`][crate::register_int_counter_vec_with_registry]: Registers an integer counter vector with a custom registry
/// - [`int_counter!`][crate::int_counter]: Creates an integer counter without registering it
#[macro_export(local_inner_macros)]
macro_rules! register_int_counter_with_registry {
    ($OPTS:expr, $REGISTRY:expr $(,)?) => {{
        register_counter_with_registry!(@of_type IntCounter, $OPTS, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $REGISTRY:expr $(,)?) => {{
        register_int_counter_with_registry!(opts!($NAME, $HELP), $REGISTRY)
    }};
}

#[test]
fn test_register_int_counter() {
    use crate::Registry;
    use std::collections::HashMap;

    let opts = opts!("test_opts_int_counter_1", "help");
    let res = register_int_counter!(opts);
    assert!(res.is_ok());

    let res = register_int_counter!("test_opts_int_counter_2", "help");
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_counter_3", "help",);
    let res = register_int_counter!(opts,);
    assert!(res.is_ok());

    let res = register_int_counter!("test_opts_int_counter_4", "help",);
    assert!(res.is_ok());

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_opts_int_counter_1", "help");
    let res = register_int_counter_with_registry!(opts, custom_registry);
    assert!(res.is_ok());

    let res =
        register_int_counter_with_registry!("test_opts_int_counter_2", "help", custom_registry);
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_counter_3", "help");
    let res = register_int_counter_with_registry!(opts, custom_registry,);
    assert!(res.is_ok());

    let res =
        register_int_counter_with_registry!("test_opts_int_counter_4", "help", custom_registry,);
    assert!(res.is_ok());
}

#[macro_export]
#[doc(hidden)]
macro_rules! __register_counter_vec {
    ($TYPE:ident, $OPTS:expr, $LABELS_NAMES:expr) => {{
        let counter_vec = $crate::$TYPE::new($OPTS, $LABELS_NAMES).unwrap();
        $crate::register(Box::new(counter_vec.clone())).map(|()| counter_vec)
    }};

    ($TYPE:ident, $OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr) => {{
        let counter_vec = $crate::$TYPE::new($OPTS, $LABELS_NAMES).unwrap();
        $REGISTRY
            .register(Box::new(counter_vec.clone()))
            .map(|()| counter_vec)
    }};
}

/// Creates a [`CounterVec`][crate::CounterVec] and registers it to the default registry.
///
/// A [`CounterVec`][crate::CounterVec] represents a collection of counters with the same name
/// but different label values. Counters in Prometheus are cumulative metrics that represent
/// monotonically increasing values that can only go up or be reset to zero on process restart.
///
/// Counter vectors are particularly useful for tracking counts across multiple dimensions,
/// such as HTTP requests by endpoint and status code, or errors by type and component.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_counter_vec!(opts, label_names);
///
/// // Using name and help strings directly:
/// register_counter_vec!(name, help, label_names);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the counter's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `label_names`: A slice of string slices (`&[&str]`) defining the label dimensions for the counter vector.
///
/// # Return Value
///
/// Returns a `Result<CounterVec, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered.
///
/// # Examples
///
/// Using with pre-defined options:
///
/// ```rust
/// use prometheus::{register_counter_vec, opts, labels};
///
/// // Create options with constant labels for the service
/// let opts = opts!(
///     "http_requests_total",
///     "Total number of HTTP requests",
///     labels!{"service" => "api"}
/// );
///
/// // Create and register the counter vector
/// let http_requests = register_counter_vec!(
///     opts,
///     &["method", "endpoint", "status"]
/// ).unwrap();
///
/// // Increment counters for specific requests
/// http_requests.with_label_values(&["GET", "/users", "200"]).inc();
/// http_requests.with_label_values(&["POST", "/orders", "201"]).inc();
/// http_requests.with_label_values(&["GET", "/products", "404"]).inc();
///
/// // Increment by a specific amount
/// http_requests.with_label_values(&["GET", "/users", "200"]).inc_by(4.0);
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::register_counter_vec;
///
/// // Create and register the counter vector directly
/// let database_errors = register_counter_vec!(
///     "database_errors_total",
///     "Total number of database errors",
///     &["operation", "database", "error_type"]
/// ).unwrap();
///
/// // Increment counters
/// database_errors.with_label_values(&["insert", "users", "connection_timeout"]).inc();
/// database_errors.with_label_values(&["select", "products", "query_timeout"]).inc();
/// ```
///
/// # Related Macros
///
/// - [`register_counter_vec_with_registry!`][crate::register_counter_vec_with_registry]: Registers a counter vector with a custom registry
/// - [`register_int_counter_vec!`][crate::register_int_counter_vec]: Registers an integer counter vector with the default registry
/// - [`register_counter!`][crate::register_counter]: Registers a single counter with the default registry
/// - [`counter_vec!`][crate::counter_vec]: Creates a counter vector without registering it
///
/// For custom registry usage, see [`register_counter_vec_with_registry!`][crate::register_counter_vec_with_registry].
#[macro_export(local_inner_macros)]
macro_rules! register_counter_vec {
    ($OPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        __register_counter_vec!(CounterVec, $OPTS, $LABELS_NAMES)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr $(,)?) => {{
        register_counter_vec!(opts!($NAME, $HELP), $LABELS_NAMES)
    }};
}

#[test]
fn test_register_counter_vec_trailing_comma() {
    let opts = opts!("test_macro_counter_vec_1", "help",);
    let counter_vec = register_counter_vec!(opts, &["a", "b"],);
    assert!(counter_vec.is_ok());

    let counter_vec = register_counter_vec!("test_macro_counter_vec_2", "help", &["a", "b"],);
    assert!(counter_vec.is_ok());
}

/// Creates a [`CounterVec`][crate::CounterVec] and registers it with a custom registry.
///
/// This macro allows for registering counter vectors to custom registries rather than
/// the default global registry. This is particularly useful for applications that need
/// to maintain separate metric collections or use custom label sets.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_counter_vec_with_registry!(opts, label_names, registry);
///
/// // Using name and help strings directly:
/// register_counter_vec_with_registry!(name, help, label_names, registry);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the counter's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `label_names`: A slice of string slices (`&[&str]`) defining the label dimensions for the counter vector.
/// - `registry`: The [`Registry`][crate::Registry] instance where the counter will be registered.
///
/// # Return Value
///
/// Returns a `Result<CounterVec, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered.
///
/// # Examples
///
/// Using with a custom registry and pre-defined options:
///
/// ```rust
/// use prometheus::{register_counter_vec_with_registry, opts, Registry};
/// use std::collections::HashMap;
///
/// // Create a custom registry with a prefix and labels
/// let mut labels = HashMap::new();
/// labels.insert("environment".to_string(), "production".to_string());
/// let custom_registry = Registry::new_custom(
///     Some("app".to_string()),
///     Some(labels)
/// ).unwrap();
///
/// // Register using pre-defined Opts
/// let opts = opts!("http_requests_total", "Total number of HTTP requests");
/// let request_counter = register_counter_vec_with_registry!(
///     opts,
///     &["method", "endpoint", "status"],
///     custom_registry
/// ).unwrap();
///
/// // Increment a specific counter in the vector
/// request_counter.with_label_values(&["GET", "/api/users", "200"]).inc();
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::{register_counter_vec_with_registry, Registry};
///
/// let custom_registry = Registry::new();
///
/// // Register using name and help directly
/// let error_counter = register_counter_vec_with_registry!(
///     "errors_total",
///     "Total number of errors by type and module",
///     &["error_type", "module"],
///     custom_registry
/// ).unwrap();
///
/// // Increment a specific counter in the vector
/// error_counter.with_label_values(&["timeout", "database"]).inc();
/// ```
///
/// # Related Macros
///
/// - [`register_counter_vec!`][crate::register_counter_vec]: Registers a counter vector with the default registry
/// - [`counter_vec!`][crate::counter_vec]: Creates a counter vector without registering it
/// - [`opts!`][crate::opts]: Creates an Opts instance for metric configuration
#[macro_export(local_inner_macros)]
macro_rules! register_counter_vec_with_registry {
    ($OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        __register_counter_vec!(CounterVec, $OPTS, $LABELS_NAMES, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        register_counter_vec_with_registry!(opts!($NAME, $HELP), $LABELS_NAMES, $REGISTRY)
    }};
}

#[test]
fn test_register_counter_vec_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_macro_counter_vec_1", "help",);
    let counter_vec = register_counter_vec_with_registry!(opts, &["a", "b"], custom_registry,);
    assert!(counter_vec.is_ok());

    let counter_vec = register_counter_vec_with_registry!(
        "test_macro_counter_vec_2",
        "help",
        &["a", "b"],
        custom_registry,
    );
    assert!(counter_vec.is_ok());
}

/// Creates an [`IntCounterVec`][crate::IntCounterVec] and registers it to the default registry.
///
/// This macro creates a vector of integer-only counters that are optimized for performance
/// when you don't need floating-point precision. Unlike the standard [`CounterVec`][crate::CounterVec],
/// the [`IntCounterVec`][crate::IntCounterVec] only accepts integer increments.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_int_counter_vec!(opts, label_names);
///
/// // Using name and help strings directly:
/// register_int_counter_vec!(name, help, label_names);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the counter's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `label_names`: A slice of string slices (`&[&str]`) defining the label dimensions for the counter vector.
///
/// # Return Value
///
/// Returns a `Result<IntCounterVec, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered.
///
/// # Examples
///
/// ```rust
/// use prometheus::{register_int_counter_vec, opts};
///
/// // Using pre-defined Opts
/// let opts = opts!("database_operations_total", "Total number of database operations");
/// let db_ops_counter = register_int_counter_vec!(opts, &["operation", "table"]).unwrap();
///
/// // Increment by 1
/// db_ops_counter.with_label_values(&["insert", "users"]).inc();
///
/// // Increment by specific amount
/// db_ops_counter.with_label_values(&["select", "products"]).inc_by(42);
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::register_int_counter_vec;
///
/// // Register using name and help directly
/// let request_counter = register_int_counter_vec!(
///     "http_requests_total",
///     "Total number of HTTP requests processed",
///     &["method", "endpoint", "status"]
/// ).unwrap();
///
/// // Increment counters
/// request_counter.with_label_values(&["GET", "/api/users", "200"]).inc();
/// request_counter.with_label_values(&["POST", "/api/orders", "201"]).inc_by(1);
/// ```
///
/// # Related Macros
///
/// - [`register_counter_vec!`][crate::register_counter_vec]: Registers a floating-point counter vector with the default registry
/// - [`register_int_counter_vec_with_registry!`][crate::register_int_counter_vec_with_registry]: Registers an integer counter vector with a custom registry
/// - [`int_counter_vec!`][crate::int_counter_vec]: Creates an integer counter vector without registering it
///
/// For custom registry usage, see [`register_int_counter_vec_with_registry!`][crate::register_int_counter_vec_with_registry].
#[macro_export(local_inner_macros)]
macro_rules! register_int_counter_vec {
    ($OPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        __register_counter_vec!(IntCounterVec, $OPTS, $LABELS_NAMES)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr $(,)?) => {{
        register_int_counter_vec!(opts!($NAME, $HELP), $LABELS_NAMES)
    }};
}

/// Creates an [`IntCounterVec`][crate::IntCounterVec] and registers it with a custom registry.
///
/// This macro allows for registering integer-only counter vectors to custom registries rather than
/// the default global registry. [`IntCounterVec`][crate::IntCounterVec] is optimized for performance
/// when you don't need floating-point precision and only work with integer increments.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_int_counter_vec_with_registry!(opts, label_names, registry);
///
/// // Using name and help strings directly:
/// register_int_counter_vec_with_registry!(name, help, label_names, registry);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the counter's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `label_names`: A slice of string slices (`&[&str]`) defining the label dimensions for the counter vector.
/// - `registry`: The [`Registry`][crate::Registry] instance where the counter will be registered.
///
/// # Return Value
///
/// Returns a `Result<IntCounterVec, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered in the specified registry.
///
/// # Examples
///
/// Using with a custom registry and pre-defined options:
///
/// ```rust
/// use prometheus::{register_int_counter_vec_with_registry, opts, Registry};
/// use std::collections::HashMap;
///
/// // Create a custom registry with a prefix and labels
/// let mut labels = HashMap::new();
/// labels.insert("service".to_string(), "api_gateway".to_string());
/// let custom_registry = Registry::new_custom(
///     Some("app".to_string()),
///     Some(labels)
/// ).unwrap();
///
/// // Register using pre-defined Opts
/// let opts = opts!("request_bytes_total", "Total bytes received in requests");
/// let bytes_counter = register_int_counter_vec_with_registry!(
///     opts,
///     &["endpoint", "method"],
///     custom_registry
/// ).unwrap();
///
/// // Increment by specific integer amount
/// bytes_counter.with_label_values(&["/api/users", "POST"]).inc_by(1024);
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::{register_int_counter_vec_with_registry, Registry};
///
/// let custom_registry = Registry::new();
///
/// // Register using name and help directly
/// let operation_counter = register_int_counter_vec_with_registry!(
///     "operations_total",
///     "Total number of operations performed by type",
///     &["operation_type", "status"],
///     custom_registry
/// ).unwrap();
///
/// // Increment counters
/// operation_counter.with_label_values(&["database_query", "success"]).inc();
/// operation_counter.with_label_values(&["cache_lookup", "miss"]).inc_by(5);
/// ```
///
/// # Related Macros
///
/// - [`register_int_counter_vec!`][crate::register_int_counter_vec]: Registers an integer counter vector with the default registry
/// - [`register_counter_vec_with_registry!`][crate::register_counter_vec_with_registry]: Registers a floating-point counter vector with a custom registry
/// - [`int_counter_vec!`][crate::int_counter_vec]: Creates an integer counter vector without registering it
#[macro_export(local_inner_macros)]
macro_rules! register_int_counter_vec_with_registry {
    ($OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        __register_counter_vec!(IntCounterVec, $OPTS, $LABELS_NAMES, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        register_int_counter_vec_with_registry!(opts!($NAME, $HELP), $LABELS_NAMES, $REGISTRY)
    }};
}

#[test]
fn test_register_int_counter_vec() {
    use crate::Registry;
    use std::collections::HashMap;

    let opts = opts!("test_opts_int_counter_vec_1", "help");
    let res = register_int_counter_vec!(opts, &["a", "b"]);
    assert!(res.is_ok());

    let res = register_int_counter_vec!("test_opts_int_counter_vec_2", "help", &["a", "b"]);
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_counter_vec_3", "help",);
    let res = register_int_counter_vec!(opts, &["a", "b"],);
    assert!(res.is_ok());

    let res = register_int_counter_vec!("test_opts_int_counter_vec_4", "help", &["a", "b"],);
    assert!(res.is_ok());

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_opts_int_counter_vec_1", "help");
    let res = register_int_counter_vec_with_registry!(opts, &["a", "b"], custom_registry);
    assert!(res.is_ok());

    let res = register_int_counter_vec_with_registry!(
        "test_opts_int_counter_vec_2",
        "help",
        &["a", "b"],
        custom_registry
    );
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_counter_vec_3", "help");
    let res = register_int_counter_vec_with_registry!(opts, &["a", "b"], custom_registry,);
    assert!(res.is_ok());

    let res = register_int_counter_vec_with_registry!(
        "test_opts_int_counter_vec_4",
        "help",
        &["a", "b"],
        custom_registry,
    );
    assert!(res.is_ok());
}

#[macro_export]
#[doc(hidden)]
macro_rules! __register_gauge {
    ($TYPE:ident, $OPTS:expr) => {{
        let gauge = $crate::$TYPE::with_opts($OPTS).unwrap();
        $crate::register(Box::new(gauge.clone())).map(|()| gauge)
    }};

    ($TYPE:ident, $OPTS:expr, $REGISTRY:expr) => {{
        let gauge = $crate::$TYPE::with_opts($OPTS).unwrap();
        $REGISTRY.register(Box::new(gauge.clone())).map(|()| gauge)
    }};
}

/// Creates a [`Gauge`][crate::Gauge] and registers it to the default registry.
///
/// A gauge is a metric that represents a single numerical value that can arbitrarily go up and down.
/// Gauges are typically used for measured values like temperatures, current memory usage,
/// or the number of active connections.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_gauge!(opts);
///
/// // Using name and help strings directly:
/// register_gauge!(name, help);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the gauge's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
///
/// # Return Value
///
/// Returns a `Result<Gauge, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered.
///
/// # Examples
///
/// Using with pre-defined options:
///
/// ```rust
/// use prometheus::{register_gauge, opts};
///
/// // Create and register the gauge
/// let opts = opts!("process_memory_bytes", "Current memory usage in bytes");
/// let memory_gauge = register_gauge!(opts).unwrap();
///
/// // Update the gauge value
/// memory_gauge.set(150_000_000.0); // Set to 150MB
/// memory_gauge.inc();              // Increment by 1
/// memory_gauge.dec_by(50_000.0);   // Decrease by 50KB
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::register_gauge;
///
/// // Create and register the gauge directly
/// let cpu_temp = register_gauge!(
///     "cpu_temperature_celsius",
///     "Current CPU temperature in Celsius"
/// ).unwrap();
///
/// // Set and adjust values
/// cpu_temp.set(45.5);  // Set the initial value
/// cpu_temp.inc_by(2.7); // Increase by 2.7
/// ```
///
/// # Related Macros
///
/// - [`register_gauge_vec!`][crate::register_gauge_vec]: Registers a gauge vector with the default registry
/// - [`register_gauge_with_registry!`][crate::register_gauge_with_registry]: Registers a gauge with a custom registry
/// - [`gauge!`][crate::gauge]: Creates a gauge without registering it
/// - [`opts!`][crate::opts]: Creates an Opts instance for metric configuration
///
/// For custom registry usage, see [`register_gauge_with_registry!`][crate::register_gauge_with_registry].
#[macro_export(local_inner_macros)]
macro_rules! register_gauge {
    ($OPTS:expr $(,)?) => {{
        __register_gauge!(Gauge, $OPTS)
    }};

    ($NAME:expr, $HELP:expr $(,)?) => {{
        register_gauge!(opts!($NAME, $HELP))
    }};
}

#[test]
fn test_register_gauge_trailing_comma() {
    let opts = opts!("test_macro_gauge", "help",);
    let res1 = register_gauge!(opts,);
    assert!(res1.is_ok());

    let res2 = register_gauge!("test_macro_gauge_2", "help",);
    assert!(res2.is_ok());
}

/// Creates a [`Gauge`][crate::Gauge] and registers it with a custom registry.
///
/// This macro allows for registering gauges to custom registries rather than
/// the default global registry. This is particularly useful for applications that need
/// to maintain separate metric collections or use custom label sets.
///
/// A gauge is a metric that represents a single numerical value that can arbitrarily go up and down.
/// Gauges are typically used for measured values like temperatures, current memory usage,
/// or the number of active connections.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_gauge_with_registry!(opts, registry);
///
/// // Using name and help strings directly:
/// register_gauge_with_registry!(name, help, registry);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the gauge's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `registry`: The [`Registry`][crate::Registry] instance where the gauge will be registered.
///
/// # Return Value
///
/// Returns a `Result<Gauge, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered in the specified registry.
///
/// # Examples
///
/// Using with a custom registry and pre-defined options:
///
/// ```rust
/// use prometheus::{register_gauge_with_registry, opts, Registry};
/// use std::collections::HashMap;
///
/// // Create a custom registry with a prefix and labels
/// let mut labels = HashMap::new();
/// labels.insert("environment".to_string(), "production".to_string());
/// let custom_registry = Registry::new_custom(
///     Some("app".to_string()),
///     Some(labels)
/// ).unwrap();
///
/// // Create and register the gauge
/// let opts = opts!("system_memory_usage_bytes", "Current system memory usage in bytes");
/// let memory_gauge = register_gauge_with_registry!(opts, custom_registry).unwrap();
///
/// // Update the gauge value
/// memory_gauge.set(256_000_000.0); // Set to 256MB
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::{register_gauge_with_registry, Registry};
///
/// // Create a simple custom registry
/// let custom_registry = Registry::new();
///
/// // Create and register the gauge directly
/// let temp_gauge = register_gauge_with_registry!(
///     "server_temperature_celsius",
///     "Current server temperature in Celsius",
///     custom_registry
/// ).unwrap();
///
/// // Set and adjust values
/// temp_gauge.set(32.5);   // Set the initial value
/// temp_gauge.dec_by(1.2); // Decrease by 1.2
/// ```
///
/// # Related Macros
///
/// - [`register_gauge!`][crate::register_gauge]: Registers a gauge with the default registry
/// - [`register_gauge_vec_with_registry!`][crate::register_gauge_vec_with_registry]: Registers a gauge vector with a custom registry
/// - [`gauge!`][crate::gauge]: Creates a gauge without registering it
/// - [`opts!`][crate::opts]: Creates an Opts instance for metric configuration
#[macro_export(local_inner_macros)]
macro_rules! register_gauge_with_registry {
    ($OPTS:expr, $REGISTRY:expr $(,)?) => {{
        __register_gauge!(Gauge, $OPTS, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $REGISTRY:expr $(,)?) => {{
        register_gauge_with_registry!(opts!($NAME, $HELP), $REGISTRY)
    }};
}

#[test]
fn test_register_gauge_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_macro_gauge", "help",);
    let res1 = register_gauge_with_registry!(opts, custom_registry,);
    assert!(res1.is_ok());

    let res2 = register_gauge_with_registry!("test_macro_gauge_2", "help", custom_registry,);
    assert!(res2.is_ok());
}

/// Creates an [`IntGauge`][crate::IntGauge] and registers it to the default registry.
///
/// An [`IntGauge`][crate::IntGauge] is a specialized version of [`Gauge`][crate::Gauge] that only accepts
/// integer values. It's optimized for performance when you don't need floating-point precision.
/// Like regular gauges, an IntGauge represents a single numerical value that can arbitrarily go up and down.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_int_gauge!(opts);
///
/// // Using name and help strings directly:
/// register_int_gauge!(name, help);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the gauge's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
///
/// # Return Value
///
/// Returns a `Result<IntGauge, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered.
///
/// # Examples
///
/// Using with pre-defined options:
///
/// ```rust
/// use prometheus::{register_int_gauge, opts};
///
/// // Create and register the int gauge
/// let opts = opts!("active_connections", "Number of currently active connections");
/// let connections = register_int_gauge!(opts).unwrap();
///
/// // Update the gauge value
/// connections.set(42);     // Set to 42
/// connections.inc();       // Increment by 1
/// connections.dec_by(5);   // Decrease by 5
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::register_int_gauge;
///
/// // Create and register the int gauge directly
/// let queue_size = register_int_gauge!(
///     "task_queue_size",
///     "Current number of tasks in the queue"
/// ).unwrap();
///
/// // Set and adjust values
/// queue_size.set(0);     // Initialize to empty
/// queue_size.inc_by(10); // Add 10 tasks
/// queue_size.dec();      // Remove 1 task
/// ```
///
/// # Related Macros
///
/// - [`register_gauge!`][crate::register_gauge]: Registers a floating-point gauge with the default registry
/// - [`register_int_gauge_vec!`][crate::register_int_gauge_vec]: Registers an integer gauge vector with the default registry
/// - [`register_int_gauge_with_registry!`][crate::register_int_gauge_with_registry]: Registers an integer gauge with a custom registry
/// - [`int_gauge!`][crate::int_gauge]: Creates an integer gauge without registering it
///
/// For custom registry usage, see [`register_int_gauge_with_registry!`][crate::register_int_gauge_with_registry].
#[macro_export(local_inner_macros)]
macro_rules! register_int_gauge {
    ($OPTS:expr $(,)?) => {{
        __register_gauge!(IntGauge, $OPTS)
    }};

    ($NAME:expr, $HELP:expr $(,)?) => {{
        register_int_gauge!(opts!($NAME, $HELP))
    }};
}

/// Creates an [`IntGauge`][crate::IntGauge] and registers it with a custom registry.
///
/// This macro allows for registering integer gauges to custom registries rather than
/// the default global registry. An [`IntGauge`][crate::IntGauge] is a specialized version of
/// [`Gauge`][crate::Gauge] that only accepts integer values and is optimized for performance
/// when floating-point precision is not needed.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_int_gauge_with_registry!(opts, registry);
///
/// // Using name and help strings directly:
/// register_int_gauge_with_registry!(name, help, registry);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the gauge's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `registry`: The [`Registry`][crate::Registry] instance where the gauge will be registered.
///
/// # Return Value
///
/// Returns a `Result<IntGauge, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered in the specified registry.
///
/// # Examples
///
/// Using with a custom registry and pre-defined options:
///
/// ```rust
/// use prometheus::{register_int_gauge_with_registry, opts, Registry};
/// use std::collections::HashMap;
///
/// // Create a custom registry with a prefix and labels
/// let mut labels = HashMap::new();
/// labels.insert("service".to_string(), "auth".to_string());
/// let custom_registry = Registry::new_custom(
///     Some("api".to_string()),
///     Some(labels)
/// ).unwrap();
///
/// // Create and register the int gauge
/// let opts = opts!("user_sessions", "Number of active user sessions");
/// let sessions = register_int_gauge_with_registry!(opts, custom_registry).unwrap();
///
/// // Update the gauge value
/// sessions.set(120);    // Set to 120
/// sessions.inc_by(15);  // Add 15 more sessions
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::{register_int_gauge_with_registry, Registry};
///
/// // Create a simple custom registry
/// let custom_registry = Registry::new();
///
/// // Create and register the int gauge directly
/// let workers = register_int_gauge_with_registry!(
///     "worker_count",
///     "Number of active worker threads",
///     custom_registry
/// ).unwrap();
///
/// // Set and adjust values
/// workers.set(8);   // Start with 8 workers
/// workers.inc();    // Add one worker
/// workers.dec_by(2); // Remove two workers
/// ```
///
/// # Related Macros
///
/// - [`register_int_gauge!`][crate::register_int_gauge]: Registers an integer gauge with the default registry
/// - [`register_gauge_with_registry!`][crate::register_gauge_with_registry]: Registers a floating-point gauge with a custom registry
/// - [`register_int_gauge_vec_with_registry!`][crate::register_int_gauge_vec_with_registry]: Registers an integer gauge vector with a custom registry
/// - [`int_gauge!`][crate::int_gauge]: Creates an integer gauge without registering it
#[macro_export(local_inner_macros)]
macro_rules! register_int_gauge_with_registry {
    ($OPTS:expr, $REGISTRY:expr $(,)?) => {{
        __register_gauge!(IntGauge, $OPTS, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $REGISTRY:expr $(,)?) => {{
        register_int_gauge_with_registry!(opts!($NAME, $HELP), $REGISTRY)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __register_gauge_vec {
    ($TYPE:ident, $OPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        let gauge_vec = $crate::$TYPE::new($OPTS, $LABELS_NAMES).unwrap();
        $crate::register(Box::new(gauge_vec.clone())).map(|()| gauge_vec)
    }};

    ($TYPE:ident, $OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        let gauge_vec = $crate::$TYPE::new($OPTS, $LABELS_NAMES).unwrap();
        $REGISTRY
            .register(Box::new(gauge_vec.clone()))
            .map(|()| gauge_vec)
    }};
}

#[test]
fn test_register_int_gauge() {
    use crate::Registry;
    use std::collections::HashMap;

    let opts = opts!("test_opts_int_gauge_1", "help");
    let res = register_int_gauge!(opts);
    assert!(res.is_ok());

    let res = register_int_gauge!("test_opts_int_gauge_2", "help");
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_gauge_3", "help",);
    let res = register_int_gauge!(opts,);
    assert!(res.is_ok());

    let res = register_int_gauge!("test_opts_int_gauge_4", "help",);
    assert!(res.is_ok());

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_opts_int_gauge_1", "help");
    let res = register_int_gauge_with_registry!(opts, custom_registry);
    assert!(res.is_ok());

    let res = register_int_gauge_with_registry!("test_opts_int_gauge_2", "help", custom_registry);
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_gauge_3", "help");
    let res = register_int_gauge_with_registry!(opts, custom_registry,);
    assert!(res.is_ok());

    let res = register_int_gauge_with_registry!("test_opts_int_gauge_4", "help", custom_registry,);
    assert!(res.is_ok());
}

/// Creates a [`GaugeVec`][crate::GaugeVec] and registers it to the default registry.
///
/// A [`GaugeVec`][crate::GaugeVec] represents a collection of gauges with the same name but different label values.
/// Gauges measure values that can go up and down, such as temperatures, memory usage, or concurrent requests.
/// The vector form allows tracking these values across different dimensions (e.g., per service, per endpoint).
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_gauge_vec!(opts, label_names);
///
/// // Using name and help strings directly:
/// register_gauge_vec!(name, help, label_names);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the gauge's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `label_names`: A slice of string slices (`&[&str]`) defining the label dimensions for the gauge vector.
///
/// # Return Value
///
/// Returns a `Result<GaugeVec, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered.
///
/// # Examples
///
/// Using with pre-defined options:
///
/// ```rust
/// use prometheus::{register_gauge_vec, opts};
///
/// // Create and register the gauge vector
/// let opts = opts!("service_memory_usage_bytes", "Memory usage by service and type");
/// let memory_gauge = register_gauge_vec!(
///     opts,
///     &["service", "memory_type"]
/// ).unwrap();
///
/// // Set values for specific label combinations
/// memory_gauge.with_label_values(&["auth", "heap"]).set(150_000_000.0);
/// memory_gauge.with_label_values(&["auth", "stack"]).set(10_000_000.0);
/// memory_gauge.with_label_values(&["database", "heap"]).set(800_000_000.0);
///
/// // Modify an existing labeled gauge
/// memory_gauge.with_label_values(&["auth", "heap"]).inc_by(50_000_000.0);
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::register_gauge_vec;
///
/// // Create and register the gauge vector directly
/// let temp_gauge = register_gauge_vec!(
///     "equipment_temperature_celsius",
///     "Equipment temperature in Celsius by location and device",
///     &["location", "device"]
/// ).unwrap();
///
/// // Set values for specific equipment
/// temp_gauge.with_label_values(&["server_room", "cpu"]).set(45.2);
/// temp_gauge.with_label_values(&["server_room", "disk"]).set(38.7);
/// temp_gauge.with_label_values(&["office", "thermostat"]).set(22.5);
///
/// // Adjust a value
/// temp_gauge.with_label_values(&["server_room", "cpu"]).dec_by(2.1);
/// ```
///
/// # Related Macros
///
/// - [`register_gauge!`][crate::register_gauge]: Registers a single gauge with the default registry
/// - [`register_gauge_vec_with_registry!`][crate::register_gauge_vec_with_registry]: Registers a gauge vector with a custom registry
/// - [`register_int_gauge_vec!`][crate::register_int_gauge_vec]: Registers an integer gauge vector with the default registry
/// - [`gauge_vec!`][crate::gauge_vec]: Creates a gauge vector without registering it
///
/// For custom registry usage, see [`register_gauge_vec_with_registry!`][crate::register_gauge_vec_with_registry].
#[macro_export(local_inner_macros)]
macro_rules! register_gauge_vec {
    ($OPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        __register_gauge_vec!(GaugeVec, $OPTS, $LABELS_NAMES)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr $(,)?) => {{
        register_gauge_vec!(opts!($NAME, $HELP), $LABELS_NAMES)
    }};
}

#[test]
fn test_register_gauge_vec_trailing_comma() {
    let opts = opts!("test_macro_gauge_vec_1", "help",);
    let gauge_vec = register_gauge_vec!(opts, &["a", "b"],);
    assert!(gauge_vec.is_ok());

    let gauge_vec = register_gauge_vec!("test_macro_gauge_vec_2", "help", &["a", "b"],);
    assert!(gauge_vec.is_ok());
}

/// Creates a [`GaugeVec`][crate::GaugeVec] and registers it with a custom registry.
///
/// This macro allows for registering gauge vectors to custom registries rather than
/// the default global registry. A [`GaugeVec`][crate::GaugeVec] represents a collection of gauges
/// with the same name but different label values. Using a custom registry is particularly
/// useful for maintaining separate metric collections, such as per-component metrics or metrics
/// with different global label sets.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_gauge_vec_with_registry!(opts, label_names, registry);
///
/// // Using name and help strings directly:
/// register_gauge_vec_with_registry!(name, help, label_names, registry);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the gauge's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `label_names`: A slice of string slices (`&[&str]`) defining the label dimensions for the gauge vector.
/// - `registry`: The [`Registry`][crate::Registry] instance where the gauge vector will be registered.
///
/// # Return Value
///
/// Returns a `Result<GaugeVec, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered in the specified registry.
///
/// # Examples
///
/// Using with a custom registry and pre-defined options:
///
/// ```rust
/// use prometheus::{register_gauge_vec_with_registry, opts, Registry};
/// use std::collections::HashMap;
///
/// // Create a custom registry with a prefix and labels
/// let mut labels = HashMap::new();
/// labels.insert("datacenter".to_string(), "east-1".to_string());
/// labels.insert("environment".to_string(), "production".to_string());
/// let custom_registry = Registry::new_custom(
///     Some("myapp".to_string()),
///     Some(labels)
/// ).unwrap();
///
/// // Create and register the gauge vector
/// let opts = opts!("api_response_time_seconds", "API response time in seconds");
/// let response_time = register_gauge_vec_with_registry!(
///     opts,
///     &["endpoint", "method"],
///     custom_registry
/// ).unwrap();
///
/// // Set values for specific endpoints and methods
/// response_time.with_label_values(&["/users", "GET"]).set(0.042);
/// response_time.with_label_values(&["/orders", "POST"]).set(0.157);
/// response_time.with_label_values(&["/items", "GET"]).set(0.089);
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::{register_gauge_vec_with_registry, Registry};
///
/// // Create a simple custom registry
/// let service_registry = Registry::new();
///
/// // Create and register the gauge vector directly
/// let connection_gauge = register_gauge_vec_with_registry!(
///     "db_connections",
///     "Database connections by pool and state",
///     &["pool", "state"],
///     service_registry
/// ).unwrap();
///
/// // Set values for different connection pools and states
/// connection_gauge.with_label_values(&["primary", "active"]).set(42.0);
/// connection_gauge.with_label_values(&["primary", "idle"]).set(10.0);
/// connection_gauge.with_label_values(&["replica", "active"]).set(18.0);
/// connection_gauge.with_label_values(&["replica", "idle"]).set(5.0);
///
/// // Update a specific gauge
/// connection_gauge.with_label_values(&["primary", "active"]).inc_by(3.0);
/// ```
///
/// # Related Macros
///
/// - [`register_gauge_vec!`][crate::register_gauge_vec]: Registers a gauge vector with the default registry
/// - [`register_gauge_with_registry!`][crate::register_gauge_with_registry]: Registers a single gauge with a custom registry
/// - [`register_int_gauge_vec_with_registry!`][crate::register_int_gauge_vec_with_registry]: Registers an integer gauge vector with a custom registry
/// - [`gauge_vec!`][crate::gauge_vec]: Creates a gauge vector without registering it
#[macro_export(local_inner_macros)]
macro_rules! register_gauge_vec_with_registry {
    ($OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        __register_gauge_vec!(GaugeVec, $OPTS, $LABELS_NAMES, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        register_gauge_vec_with_registry!(opts!($NAME, $HELP), $LABELS_NAMES, $REGISTRY)
    }};
}

#[test]
fn test_register_gauge_vec_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_macro_gauge_vec_1", "help",);
    let gauge_vec = register_gauge_vec_with_registry!(opts, &["a", "b"], custom_registry,);
    assert!(gauge_vec.is_ok());

    let gauge_vec = register_gauge_vec_with_registry!(
        "test_macro_gauge_vec_2",
        "help",
        &["a", "b"],
        custom_registry,
    );
    assert!(gauge_vec.is_ok());
}

/// Creates an [`IntGaugeVec`][crate::IntGaugeVec] and registers it to the default registry.
///
/// An [`IntGaugeVec`][crate::IntGaugeVec] is a specialized version of [`GaugeVec`][crate::GaugeVec] that only
/// accepts integer values. It's optimized for performance when you don't need floating-point precision.
/// This vector represents a collection of integer gauges with the same name but different label values,
/// making it perfect for tracking whole-number metrics across different dimensions.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_int_gauge_vec!(opts, label_names);
///
/// // Using name and help strings directly:
/// register_int_gauge_vec!(name, help, label_names);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the gauge's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `label_names`: A slice of string slices (`&[&str]`) defining the label dimensions for the gauge vector.
///
/// # Return Value
///
/// Returns a `Result<IntGaugeVec, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered.
///
/// # Examples
///
/// Using with pre-defined options:
///
/// ```rust
/// use prometheus::{register_int_gauge_vec, opts};
///
/// // Create and register the int gauge vector
/// let opts = opts!("active_connections", "Number of active connections by service and protocol");
/// let connections = register_int_gauge_vec!(
///     opts,
///     &["service", "protocol"]
/// ).unwrap();
///
/// // Set values for specific label combinations
/// connections.with_label_values(&["auth", "http"]).set(42);
/// connections.with_label_values(&["auth", "https"]).set(128);
/// connections.with_label_values(&["database", "postgres"]).set(5);
///
/// // Modify an existing labeled gauge
/// connections.with_label_values(&["auth", "http"]).inc();  // Increment by 1
/// connections.with_label_values(&["database", "postgres"]).dec_by(2);  // Decrement by 2
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::register_int_gauge_vec;
///
/// // Create and register the int gauge vector directly
/// let queue_sizes = register_int_gauge_vec!(
///     "queue_size",
///     "Number of items in various queues",
///     &["queue_name", "priority"]
/// ).unwrap();
///
/// // Set values for different queues
/// queue_sizes.with_label_values(&["notifications", "high"]).set(5);
/// queue_sizes.with_label_values(&["notifications", "low"]).set(42);
/// queue_sizes.with_label_values(&["emails", "high"]).set(3);
/// queue_sizes.with_label_values(&["emails", "low"]).set(15);
///
/// // Update a specific queue size
/// queue_sizes.with_label_values(&["emails", "low"]).inc_by(7);
/// ```
///
/// # Related Macros
///
/// - [`register_gauge_vec!`][crate::register_gauge_vec]: Registers a floating-point gauge vector with the default registry
/// - [`register_int_gauge_vec_with_registry!`][crate::register_int_gauge_vec_with_registry]: Registers an integer gauge vector with a custom registry
/// - [`register_int_gauge!`][crate::register_int_gauge]: Registers a single integer gauge with the default registry
/// - [`int_gauge_vec!`][crate::int_gauge_vec]: Creates an integer gauge vector without registering it
///
/// For custom registry usage, see [`register_int_gauge_vec_with_registry!`][crate::register_int_gauge_vec_with_registry].
#[macro_export(local_inner_macros)]
macro_rules! register_int_gauge_vec {
    ($OPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        __register_gauge_vec!(IntGaugeVec, $OPTS, $LABELS_NAMES)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr $(,)?) => {{
        register_int_gauge_vec!(opts!($NAME, $HELP), $LABELS_NAMES)
    }};
}

/// Creates an [`IntGaugeVec`][crate::IntGaugeVec] and registers it with a custom registry.
///
/// This macro allows for registering integer gauge vectors to custom registries rather than
/// the default global registry. An [`IntGaugeVec`][crate::IntGaugeVec] is a specialized version
/// of [`GaugeVec`][crate::GaugeVec] that only accepts integer values and is optimized for performance
/// when floating-point precision is not needed.
///
/// # Syntax
///
/// This macro can be used in two ways:
///
/// ```
/// // Using a pre-defined Opts struct:
/// register_int_gauge_vec_with_registry!(opts, label_names, registry);
///
/// // Using name and help strings directly:
/// register_int_gauge_vec_with_registry!(name, help, label_names, registry);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`Opts`][crate::Opts] that defines the gauge's name and help text.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `label_names`: A slice of string slices (`&[&str]`) defining the label dimensions for the gauge vector.
/// - `registry`: The [`Registry`][crate::Registry] instance where the gauge vector will be registered.
///
/// # Return Value
///
/// Returns a `Result<IntGaugeVec, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered in the specified registry.
///
/// # Examples
///
/// Using with a custom registry and pre-defined options:
///
/// ```rust
/// use prometheus::{register_int_gauge_vec_with_registry, opts, Registry};
/// use std::collections::HashMap;
///
/// // Create a custom registry with a prefix and labels
/// let mut labels = HashMap::new();
/// labels.insert("region".to_string(), "us-west".to_string());
/// let app_registry = Registry::new_custom(
///     Some("microservice".to_string()),
///     Some(labels)
/// ).unwrap();
///
/// // Create and register the int gauge vector
/// let opts = opts!("worker_threads", "Number of worker threads by pool and state");
/// let workers = register_int_gauge_vec_with_registry!(
///     opts,
///     &["pool", "state"],
///     app_registry
/// ).unwrap();
///
/// // Set values for different worker pools
/// workers.with_label_values(&["request_handlers", "idle"]).set(8);
/// workers.with_label_values(&["request_handlers", "busy"]).set(24);
/// workers.with_label_values(&["background_jobs", "idle"]).set(4);
/// workers.with_label_values(&["background_jobs", "busy"]).set(2);
///
/// // Update a specific pool's count
/// workers.with_label_values(&["request_handlers", "busy"]).inc_by(3);
/// ```
///
/// Using the shorthand syntax:
///
/// ```rust
/// use prometheus::{register_int_gauge_vec_with_registry, Registry};
///
/// // Create a simple custom registry
/// let metrics_registry = Registry::new();
///
/// // Create and register the int gauge vector directly
/// let cache_items = register_int_gauge_vec_with_registry!(
///     "cache_items",
///     "Number of items in cache by type and status",
///     &["cache_type", "status"],
///     metrics_registry
/// ).unwrap();
///
/// // Set values for different caches
/// cache_items.with_label_values(&["memory", "valid"]).set(1250);
/// cache_items.with_label_values(&["memory", "expired"]).set(120);
/// cache_items.with_label_values(&["disk", "valid"]).set(5200);
/// cache_items.with_label_values(&["disk", "expired"]).set(315);
///
/// // Update a specific metric
/// cache_items.with_label_values(&["memory", "valid"]).dec_by(50);
/// ```
///
/// # Related Macros
///
/// - [`register_int_gauge_vec!`][crate::register_int_gauge_vec]: Registers an integer gauge vector with the default registry
/// - [`register_gauge_vec_with_registry!`][crate::register_gauge_vec_with_registry]: Registers a floating-point gauge vector with a custom registry
/// - [`register_int_gauge_with_registry!`][crate::register_int_gauge_with_registry]: Registers a single integer gauge with a custom registry
/// - [`int_gauge_vec!`][crate::int_gauge_vec]: Creates an integer gauge vector without registering it
#[macro_export(local_inner_macros)]
macro_rules! register_int_gauge_vec_with_registry {
    ($OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        __register_gauge_vec!(IntGaugeVec, $OPTS, $LABELS_NAMES, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        register_int_gauge_vec_with_registry!(opts!($NAME, $HELP), $LABELS_NAMES, $REGISTRY)
    }};
}

#[test]
fn test_register_int_gauge_vec() {
    use crate::Registry;
    use std::collections::HashMap;

    let opts = opts!("test_opts_int_gauge_vec_1", "help");
    let res = register_int_gauge_vec!(opts, &["a", "b"]);
    assert!(res.is_ok());

    let res = register_int_gauge_vec!("test_opts_int_gauge_vec_2", "help", &["a", "b"]);
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_gauge_vec_3", "help",);
    let res = register_int_gauge_vec!(opts, &["a", "b"],);
    assert!(res.is_ok());

    let res = register_int_gauge_vec!("test_opts_int_gauge_vec_4", "help", &["a", "b"],);
    assert!(res.is_ok());

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_opts_int_gauge_vec_1", "help");
    let res = register_int_gauge_vec_with_registry!(opts, &["a", "b"], custom_registry);
    assert!(res.is_ok());

    let res = register_int_gauge_vec_with_registry!(
        "test_opts_int_gauge_vec_2",
        "help",
        &["a", "b"],
        custom_registry
    );
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_gauge_vec_3", "help");
    let res = register_int_gauge_vec_with_registry!(opts, &["a", "b"], custom_registry,);
    assert!(res.is_ok());

    let res = register_int_gauge_vec_with_registry!(
        "test_opts_int_gauge_vec_4",
        "help",
        &["a", "b"],
        custom_registry,
    );
    assert!(res.is_ok());
}

/// Creates a [`Histogram`][crate::Histogram] and registers it to the default registry.
///
/// A histogram samples observations (usually request durations or response sizes) and counts
/// them in configurable buckets. It also provides a sum of all observed values.
///
/// Histograms are particularly useful for measuring the distribution of values, such as
/// latencies or response sizes, where understanding percentiles and outliers is important.
///
/// # Syntax
///
/// This macro can be used in three ways:
///
/// ```
/// // Using a pre-defined HistogramOpts struct:
/// register_histogram!(opts);
///
/// // Using name and help strings with default buckets:
/// register_histogram!(name, help);
///
/// // Using name, help, and custom buckets:
/// register_histogram!(name, help, buckets);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`HistogramOpts`][crate::HistogramOpts] that defines the histogram's name, help text, and buckets.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `buckets`: A vector of `f64` values (`Vec<f64>`) defining the bucket boundaries (optional, uses default buckets if not specified).
///
/// # Return Value
///
/// Returns a `Result<Histogram, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered or if the provided options are invalid.
///
/// # Examples
///
/// Using with pre-defined options:
///
/// ```rust
/// use prometheus::{register_histogram, histogram_opts};
///
/// // Create options with custom buckets focusing on small durations
/// let opts = histogram_opts!(
///     "http_request_duration_seconds",
///     "HTTP request duration in seconds",
///     vec![0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
/// );
///
/// // Create and register the histogram
/// let request_duration = register_histogram!(opts).unwrap();
///
/// // Record observed values
/// request_duration.observe(0.153); // 153ms
/// request_duration.observe(0.42);  // 420ms
/// ```
///
/// Using the shorthand syntax with default buckets:
///
/// ```rust
/// use prometheus::register_histogram;
///
/// // Create and register the histogram with default buckets
/// let response_size = register_histogram!(
///     "http_response_size_bytes",
///     "HTTP response size in bytes"
/// ).unwrap();
///
/// // Record observed values
/// response_size.observe(2310.0);
/// response_size.observe(8715.0);
/// ```
///
/// Using the shorthand syntax with custom buckets:
///
/// ```rust
/// use prometheus::register_histogram;
///
/// // Create and register the histogram with custom buckets for file sizes
/// let file_size = register_histogram!(
///     "file_size_bytes",
///     "Processed file size in bytes",
///     vec![1024.0, 10240.0, 102400.0, 1048576.0, 10485760.0, 104857600.0]
/// ).unwrap();
///
/// // Record an observed value
/// file_size.observe(523782.0);
/// ```
///
/// # Related Macros
///
/// - [`register_histogram_with_registry!`][crate::register_histogram_with_registry]: Registers a histogram with a custom registry
/// - [`register_histogram_vec!`][crate::register_histogram_vec]: Registers a histogram vector with the default registry
/// - [`histogram_opts!`][crate::histogram_opts]: Creates a HistogramOpts instance for metric configuration
/// - [`histogram!`][crate::histogram]: Creates a histogram without registering it
///
/// For custom registry usage, see [`register_histogram_with_registry!`][crate::register_histogram_with_registry].
#[macro_export(local_inner_macros)]
macro_rules! register_histogram {
    ($NAME:expr, $HELP:expr $(,)?) => {
        register_histogram!(histogram_opts!($NAME, $HELP))
    };

    ($NAME:expr, $HELP:expr, $BUCKETS:expr $(,)?) => {
        register_histogram!(histogram_opts!($NAME, $HELP, $BUCKETS))
    };

    ($HOPTS:expr $(,)?) => {{
        let histogram = $crate::Histogram::with_opts($HOPTS).unwrap();
        $crate::register(Box::new(histogram.clone())).map(|()| histogram)
    }};
}

#[test]
fn test_register_histogram_trailing_comma() {
    let opts = histogram_opts!("test_macro_histogram", "help",);
    let res1 = register_histogram!(opts,);
    assert!(res1.is_ok());

    let res2 = register_histogram!("test_macro_histogram_2", "help",);
    assert!(res2.is_ok());

    let res3 = register_histogram!("test_macro_histogram_4", "help", vec![1.0, 2.0],);
    assert!(res3.is_ok());
}

/// Creates a [`Histogram`][crate::Histogram] and registers it with a custom registry.
///
/// This macro allows for registering histograms to custom registries rather than
/// the default global registry. A histogram samples observations (usually request durations
/// or response sizes) and counts them in configurable buckets, also providing a sum of all
/// observed values.
///
/// # Syntax
///
/// This macro can be used in three ways:
///
/// ```
/// // Using a pre-defined HistogramOpts struct:
/// register_histogram_with_registry!(opts, registry);
///
/// // Using name and help strings with default buckets:
/// register_histogram_with_registry!(name, help, registry);
///
/// // Using name, help, and custom buckets:
/// register_histogram_with_registry!(name, help, buckets, registry);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`HistogramOpts`][crate::HistogramOpts] that defines the histogram's name, help text, and buckets.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `buckets`: A vector of `f64` values (`Vec<f64>`) defining the bucket boundaries (optional, uses default buckets if not specified).
/// - `registry`: The [`Registry`][crate::Registry] instance where the histogram will be registered.
///
/// # Return Value
///
/// Returns a `Result<Histogram, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered in the specified registry or if the provided options are invalid.
///
/// # Examples
///
/// Using with a custom registry and pre-defined options:
///
/// ```rust
/// use prometheus::{register_histogram_with_registry, histogram_opts, Registry};
/// use std::collections::HashMap;
///
/// // Create a custom registry with a prefix and labels
/// let mut labels = HashMap::new();
/// labels.insert("component".to_string(), "api".to_string());
/// labels.insert("environment".to_string(), "production".to_string());
/// let custom_registry = Registry::new_custom(
///     Some("app".to_string()),
///     Some(labels)
/// ).unwrap();
///
/// // Create options with custom buckets (in seconds)
/// let opts = histogram_opts!(
///     "database_query_duration_seconds",
///     "Database query duration in seconds",
///     vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]
/// );
///
/// // Create and register the histogram
/// let query_duration = register_histogram_with_registry!(opts, custom_registry).unwrap();
///
/// // Record observed values
/// query_duration.observe(0.037); // 37ms
/// query_duration.observe(0.215); // 215ms
/// ```
///
/// Using the shorthand syntax with default buckets:
///
/// ```rust
/// use prometheus::{register_histogram_with_registry, Registry};
///
/// // Create a simple custom registry
/// let metrics_registry = Registry::new();
///
/// // Create and register the histogram with default buckets
/// let request_size = register_histogram_with_registry!(
///     "http_request_size_bytes",
///     "HTTP request size in bytes",
///     metrics_registry
/// ).unwrap();
///
/// // Record observed values
/// request_size.observe(1250.0);
/// request_size.observe(8432.0);
/// ```
///
/// Using the shorthand syntax with custom buckets:
///
/// ```rust
/// use prometheus::{register_histogram_with_registry, Registry};
///
/// // Create a custom registry
/// let app_registry = Registry::new();
///
/// // Create and register the histogram with custom buckets for memory usage in MB
/// let memory_usage = register_histogram_with_registry!(
///     "process_memory_usage_megabytes",
///     "Process memory usage in megabytes",
///     vec![50.0, 100.0, 200.0, 500.0, 1000.0, 2000.0, 5000.0],
///     app_registry
/// ).unwrap();
///
/// // Record an observed value
/// memory_usage.observe(327.5);
/// ```
///
/// # Related Macros
///
/// - [`register_histogram!`][crate::register_histogram]: Registers a histogram with the default registry
/// - [`register_histogram_vec_with_registry!`][crate::register_histogram_vec_with_registry]: Registers a histogram vector with a custom registry
/// - [`histogram_opts!`][crate::histogram_opts]: Creates a HistogramOpts instance for metric configuration
/// - [`histogram!`][crate::histogram]: Creates a histogram without registering it
#[macro_export(local_inner_macros)]
macro_rules! register_histogram_with_registry {
    ($NAME:expr, $HELP:expr, $REGISTRY:expr $(,)?) => {
        register_histogram_with_registry!(histogram_opts!($NAME, $HELP), $REGISTRY)
    };

    ($NAME:expr, $HELP:expr, $BUCKETS:expr, $REGISTRY:expr $(,)?) => {
        register_histogram_with_registry!(histogram_opts!($NAME, $HELP, $BUCKETS), $REGISTRY)
    };

    ($HOPTS:expr, $REGISTRY:expr $(,)?) => {{
        let histogram = $crate::Histogram::with_opts($HOPTS).unwrap();
        $REGISTRY
            .register(Box::new(histogram.clone()))
            .map(|()| histogram)
    }};
}

#[test]
fn test_register_histogram_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = histogram_opts!("test_macro_histogram", "help",);
    let res1 = register_histogram_with_registry!(opts, custom_registry,);
    assert!(res1.is_ok());

    let res2 =
        register_histogram_with_registry!("test_macro_histogram_2", "help", custom_registry,);
    assert!(res2.is_ok());

    let res3 = register_histogram_with_registry!(
        "test_macro_histogram_4",
        "help",
        vec![1.0, 2.0],
        custom_registry,
    );
    assert!(res3.is_ok());
}

/// Creates a [`HistogramVec`][crate::HistogramVec] and registers it to the default registry.
///
/// A [`HistogramVec`][crate::HistogramVec] represents a collection of histograms with the same name
/// but different label values. Histograms measure the distribution of values within configurable buckets,
/// making them ideal for tracking latencies, sizes, or other continuous measurements across
/// different dimensions (e.g., endpoints, status codes, etc.).
///
/// # Syntax
///
/// This macro can be used in three ways:
///
/// ```
/// // Using a pre-defined HistogramOpts struct:
/// register_histogram_vec!(opts, label_names);
///
/// // Using name, help, and label names with default buckets:
/// register_histogram_vec!(name, help, label_names);
///
/// // Using name, help, label names, and custom buckets:
/// register_histogram_vec!(name, help, label_names, buckets);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`HistogramOpts`][crate::HistogramOpts] that defines the histogram's name, help text, and buckets.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `label_names`: A slice of string slices (`&[&str]`) defining the label dimensions for the histogram vector.
/// - `buckets`: A vector of `f64` values (`Vec<f64>`) defining the bucket boundaries (optional, uses default buckets if not specified).
///
/// # Return Value
///
/// Returns a `Result<HistogramVec, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered or if the provided options are invalid.
///
/// # Examples
///
/// Using with pre-defined options:
///
/// ```rust
/// use prometheus::{register_histogram_vec, histogram_opts};
///
/// // Create options with custom buckets focusing on HTTP request durations
/// let opts = histogram_opts!(
///     "http_request_duration_seconds",
///     "HTTP request duration in seconds",
///     vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
/// );
///
/// // Create and register the histogram vector
/// let request_duration = register_histogram_vec!(
///     opts,
///     &["method", "endpoint", "status"]
/// ).unwrap();
///
/// // Record observed values for specific routes
/// request_duration.with_label_values(&["GET", "/users", "200"]).observe(0.042);
/// request_duration.with_label_values(&["POST", "/orders", "201"]).observe(0.317);
/// request_duration.with_label_values(&["GET", "/products", "200"]).observe(0.189);
/// ```
///
/// Using the shorthand syntax with default buckets:
///
/// ```rust
/// use prometheus::register_histogram_vec;
///
/// // Create and register the histogram vector with default buckets
/// let response_size = register_histogram_vec!(
///     "http_response_size_bytes",
///     "HTTP response size in bytes",
///     &["endpoint", "content_type"]
/// ).unwrap();
///
/// // Record observed values
/// response_size.with_label_values(&["/users", "application/json"]).observe(2310.0);
/// response_size.with_label_values(&["/images", "image/jpeg"]).observe(56320.0);
/// ```
///
/// Using the shorthand syntax with custom buckets:
///
/// ```rust
/// use prometheus::register_histogram_vec;
///
/// // Create and register the histogram vector with custom buckets for query times
/// let query_time = register_histogram_vec!(
///     "database_query_duration_seconds",
///     "Database query duration in seconds by type and table",
///     &["query_type", "table"],
///     vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]
/// ).unwrap();
///
/// // Record observed values for different queries
/// query_time.with_label_values(&["SELECT", "users"]).observe(0.007);
/// query_time.with_label_values(&["INSERT", "orders"]).observe(0.028);
/// query_time.with_label_values(&["UPDATE", "products"]).observe(0.035);
/// ```
///
/// # Related Macros
///
/// - [`register_histogram_vec_with_registry!`][crate::register_histogram_vec_with_registry]: Registers a histogram vector with a custom registry
/// - [`register_histogram!`][crate::register_histogram]: Registers a single histogram with the default registry
/// - [`histogram_opts!`][crate::histogram_opts]: Creates a HistogramOpts instance for metric configuration
/// - [`histogram_vec!`][crate::histogram_vec]: Creates a histogram vector without registering it
///
/// For custom registry usage, see [`register_histogram_vec_with_registry!`][crate::register_histogram_vec_with_registry].
#[macro_export(local_inner_macros)]
macro_rules! register_histogram_vec {
    ($HOPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        let histogram_vec = $crate::HistogramVec::new($HOPTS, $LABELS_NAMES).unwrap();
        $crate::register(Box::new(histogram_vec.clone())).map(|()| histogram_vec)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr $(,)?) => {{
        register_histogram_vec!(histogram_opts!($NAME, $HELP), $LABELS_NAMES)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $BUCKETS:expr $(,)?) => {{
        register_histogram_vec!(histogram_opts!($NAME, $HELP, $BUCKETS), $LABELS_NAMES)
    }};
}

#[test]
fn test_register_histogram_vec_trailing_comma() {
    let opts = histogram_opts!("test_macro_histogram_vec_1", "help",);
    let histogram_vec = register_histogram_vec!(opts, &["a", "b"],);
    assert!(histogram_vec.is_ok());

    let histogram_vec = register_histogram_vec!("test_macro_histogram_vec_2", "help", &["a", "b"],);
    assert!(histogram_vec.is_ok());

    let histogram_vec = register_histogram_vec!(
        "test_macro_histogram_vec_3",
        "help",
        &["test_label"],
        vec![0.0, 1.0, 2.0],
    );
    assert!(histogram_vec.is_ok());
}

/// Creates a [`HistogramVec`][crate::HistogramVec] and registers it with a custom registry.
///
/// This macro allows for registering histogram vectors to custom registries rather than
/// the default global registry. A [`HistogramVec`][crate::HistogramVec] represents a collection
/// of histograms with the same name but different label values, allowing you to track the
/// distribution of values (like latencies or sizes) across multiple dimensions.
///
/// # Syntax
///
/// This macro can be used in three ways:
///
/// ```
/// // Using a pre-defined HistogramOpts struct:
/// register_histogram_vec_with_registry!(opts, label_names, registry);
///
/// // Using name, help, and label names with default buckets:
/// register_histogram_vec_with_registry!(name, help, label_names, registry);
///
/// // Using name, help, label names, and custom buckets:
/// register_histogram_vec_with_registry!(name, help, label_names, buckets, registry);
/// ```
///
/// # Parameters
///
/// - `opts`: An instance of [`HistogramOpts`][crate::HistogramOpts] that defines the histogram's name, help text, and buckets.
/// - `name`: A string slice (`&str`) that specifies the metric name.
/// - `help`: A string slice (`&str`) that provides documentation for the metric.
/// - `label_names`: A slice of string slices (`&[&str]`) defining the label dimensions for the histogram vector.
/// - `buckets`: A vector of `f64` values (`Vec<f64>`) defining the bucket boundaries (optional, uses default buckets if not specified).
/// - `registry`: The [`Registry`][crate::Registry] instance where the histogram vector will be registered.
///
/// # Return Value
///
/// Returns a `Result<HistogramVec, PrometheusError>`. The operation will fail if a metric
/// with the same name is already registered in the specified registry or if the provided options are invalid.
///
/// # Examples
///
/// Using with a custom registry and pre-defined options:
///
/// ```rust
/// use prometheus::{register_histogram_vec_with_registry, histogram_opts, Registry};
/// use std::collections::HashMap;
///
/// // Create a custom registry with a prefix and labels
/// let mut labels = HashMap::new();
/// labels.insert("service".to_string(), "payment-api".to_string());
/// let custom_registry = Registry::new_custom(
///     Some("company".to_string()),
///     Some(labels)
/// ).unwrap();
///
/// // Create options with custom buckets for API latency
/// let opts = histogram_opts!(
///     "api_request_duration_seconds",
///     "API request duration in seconds",
///     vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]
/// );
///
/// // Create and register the histogram vector
/// let api_latency = register_histogram_vec_with_registry!(
///     opts,
///     &["endpoint", "method", "status_code"],
///     custom_registry
/// ).unwrap();
///
/// // Record observed values for specific API calls
/// api_latency.with_label_values(&["/payments", "POST", "201"]).observe(0.157);
/// api_latency.with_label_values(&["/accounts", "GET", "200"]).observe(0.028);
/// api_latency.with_label_values(&["/transactions", "GET", "200"]).observe(0.213);
/// ```
///
/// Using the shorthand syntax with default buckets:
///
/// ```rust
/// use prometheus::{register_histogram_vec_with_registry, Registry};
///
/// // Create a simple custom registry
/// let metrics_registry = Registry::new();
///
/// // Create and register the histogram vector with default buckets
/// let file_size = register_histogram_vec_with_registry!(
///     "processed_file_size_bytes",
///     "Size of processed files in bytes",
///     &["file_type", "processing_stage"],
///     metrics_registry
/// ).unwrap();
///
/// // Record observed values
/// file_size.with_label_values(&["csv", "raw"]).observe(45230.0);
/// file_size.with_label_values(&["csv", "compressed"]).observe(12750.0);
/// file_size.with_label_values(&["json", "raw"]).observe(223450.0);
/// ```
///
/// Using the shorthand syntax with custom buckets:
///
/// ```rust
/// use prometheus::{register_histogram_vec_with_registry, Registry};
///
/// // Create a custom registry
/// let app_registry = Registry::new();
///
/// // Create and register the histogram vector with custom buckets for cache operations
/// let cache_op_time = register_histogram_vec_with_registry!(
///     "cache_operation_duration_seconds",
///     "Duration of cache operations in seconds",
///     &["operation", "cache_type"],
///     vec![0.0001, 0.0005, 0.001, 0.0025, 0.005, 0.01, 0.025, 0.05, 0.1],
///     app_registry
/// ).unwrap();
///
/// // Record observed values for different cache operations
/// cache_op_time.with_label_values(&["get", "memory"]).observe(0.00034);
/// cache_op_time.with_label_values(&["set", "memory"]).observe(0.00057);
/// cache_op_time.with_label_values(&["get", "redis"]).observe(0.00328);
/// ```
///
/// # Related Macros
///
/// - [`register_histogram_vec!`][crate::register_histogram_vec]: Registers a histogram vector with the default registry
/// - [`register_histogram_with_registry!`][crate::register_histogram_with_registry]: Registers a single histogram with a custom registry
/// - [`histogram_opts!`][crate::histogram_opts]: Creates a HistogramOpts instance for metric configuration
/// - [`histogram_vec!`][crate::histogram_vec]: Creates a histogram vector without registering it
#[macro_export(local_inner_macros)]
macro_rules! register_histogram_vec_with_registry {
    ($HOPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        let histogram_vec = $crate::HistogramVec::new($HOPTS, $LABELS_NAMES).unwrap();
        $REGISTRY
            .register(Box::new(histogram_vec.clone()))
            .map(|()| histogram_vec)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        register_histogram_vec_with_registry!(
            histogram_opts!($NAME, $HELP),
            $LABELS_NAMES,
            $REGISTRY
        )
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $BUCKETS:expr, $REGISTRY:expr $(,)?) => {{
        register_histogram_vec_with_registry!(
            histogram_opts!($NAME, $HELP, $BUCKETS),
            $LABELS_NAMES,
            $REGISTRY
        )
    }};
}

#[test]
fn test_register_histogram_vec_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = histogram_opts!("test_macro_histogram_vec_1", "help",);
    let histogram_vec = register_histogram_vec_with_registry!(opts, &["a", "b"], custom_registry,);
    assert!(histogram_vec.is_ok());

    let histogram_vec = register_histogram_vec_with_registry!(
        "test_macro_histogram_vec_2",
        "help",
        &["a", "b"],
        custom_registry,
    );
    assert!(histogram_vec.is_ok());

    let histogram_vec = register_histogram_vec_with_registry!(
        "test_macro_histogram_vec_3",
        "help",
        &["test_label"],
        vec![0.0, 1.0, 2.0],
        custom_registry,
    );
    assert!(histogram_vec.is_ok());
}
