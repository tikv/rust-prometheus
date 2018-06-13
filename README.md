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
    prometheus = "0.4"
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

The crate has a pre-generated protobuf binding file for `protobuf` v2.0, if you need use the latest version of `protobuf`, you can generate the binding file on building with the `gen` feature.

```toml
[dependencies.prometheus]
git = "https://github.com/pingcap/rust-prometheus.git"
features = ["gen"]
```

## Example

```rust
use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};

// Create a Counter.
let counter_opts = Opts::new("test_counter", "test counter help");
let counter = Counter::with_opts(counter_opts).unwrap();

// Create a Registry and register Counter.
let r = Registry::new();
r.register(Box::new(counter.clone())).unwrap();

// Inc.
counter.inc();

// Gather the metrics.
let mut buffer = vec![];
let encoder = TextEncoder::new();
let metric_familys = r.gather();
encoder.encode(&metric_familys, &mut buffer).unwrap();

// Output to the standard output.
println!("{}", String::from_utf8(buffer).unwrap());
```

[More Examples](./examples)

## Advanced

### Static Metric

Static metric helps you make metric vectors faster.

See [static-metric](./static-metric) directory for details.

## Thanks

+ [brian-brazil](https://github.com/brian-brazil)
+ [ccmtaylor](https://github.com/ccmtaylor)
+ [kamalmarhubi](https://github.com/kamalmarhubi)
+ [lucab](https://github.com/lucab)
