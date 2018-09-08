# Prometheus Rust client library

[![Build Status](https://travis-ci.org/pingcap/rust-prometheus.svg?branch=master)](https://travis-ci.org/pingcap/rust-prometheus)
[![docs.rs](https://docs.rs/prometheus/badge.svg)](https://docs.rs/prometheus)
[![crates.io](http://meritbadge.herokuapp.com/prometheus)](https://crates.io/crates/prometheus)

This is the [Rust](https://www.rust-lang.org) client library for [Prometheus](http://prometheus.io).
The main Structures and APIs are ported from [Go client](https://github.com/prometheus/client_golang).

## Usage

+ Add this to your `Cargo.toml`:

    ```toml
    [dependencies]
    prometheus = "0.5"
    ```

+ Add this to your crate in `lib.rs`:

    ```rust
    extern crate prometheus;
    ```

+ Or enable nightly feature for better performance.

    ```toml
    [dependencies.prometheus]
    git = "https://github.com/pingcap/rust-prometheus.git"
    default-features = false
    features = ["nightly"]
    ```

### Note

The crate has a pre-generated protobuf binding file for `protobuf` v2.0, if you need use the latest version of
`protobuf`, you can generate the binding file on building with the `gen` feature.

```toml
[dependencies.prometheus]
git = "https://github.com/pingcap/rust-prometheus.git"
features = ["gen"]
```

## Example

### Basic

[examples/readme_basic.rs](./examples/readme_basic.rs): Create a register and a counter, gather metrics into text
format:

```rust
extern crate prometheus;

use prometheus::prelude::*;
use prometheus::{Counter, Registry, TextEncoder};

fn main() {
    // Create a Registry
    let r = Registry::new();

    // Create a Counter and register to the registry.
    let counter = Counter::new("page_views", "Number of page views")
        .unwrap()
        .register(&r);

    // Increase counter.
    counter.inc();

    // Gather metrics.
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = r.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    // Output to the standard output.
    println!("{}", String::from_utf8(buffer).unwrap());
}
```

### Vector

[examples/readme_vec.rs](./examples/readme_vec.rs): Create a histogram vector, gather metrics into text format from
default registry:

```rust
#[macro_use]
extern crate lazy_static;
extern crate prometheus;

use prometheus::prelude::*;
use prometheus::{HistogramVec, TextEncoder};

lazy_static! {
    // Create a static HistogramVec and register to the default registry.
    static ref PAGE_DURATION: HistogramVec<[&'static str; 2]> = HistogramVec::from_opts((
        "page_view_duration",
        "Page view duration",
        ["country", "city"],
        prometheus::exponential_buckets(0.1, 2.0, 10).unwrap(),
    ))
        .unwrap()
        .register_default();
}

fn main() {
    PAGE_DURATION
        .with_label_values(["US", "New York"])
        .observe(10.5);
    PAGE_DURATION
        .with_label_values(["US", "Los Angeles"])
        .observe(12.5);
    PAGE_DURATION
        .with_label_values(["US", "Los Angeles"])
        .observe(5.0);
    PAGE_DURATION
        .with_label_values(["China", "Shanghai"])
        .observe(5.5);
    PAGE_DURATION
        .with_label_values(["China", "Beijing"])
        .observe(7.0);

    // Gather metrics from default registry.
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    // Output to the standard output.
    println!("{}", String::from_utf8(buffer).unwrap());
}
```

### More Examples

Please refer to the [examples](./examples) directory.

## Advanced

### Static Metric

Static metric helps you make metric vectors faster.

See [static-metric](./static-metric) directory for details.

## Thanks

+ [brian-brazil](https://github.com/brian-brazil)
+ [ccmtaylor](https://github.com/ccmtaylor)
+ [kamalmarhubi](https://github.com/kamalmarhubi)
+ [lucab](https://github.com/lucab)
