# Prometheus Rust client library

[![Build Status](https://travis-ci.org/tikv/rust-prometheus.svg?branch=master)](https://travis-ci.org/pingcap/rust-prometheus)
[![docs.rs](https://docs.rs/prometheus/badge.svg)](https://docs.rs/prometheus)
[![crates.io](http://meritbadge.herokuapp.com/prometheus)](https://crates.io/crates/prometheus)

This is the [Rust](https://www.rust-lang.org) client library for
[Prometheus](http://prometheus.io). The main data structures and APIs are ported
from [Go client](https://github.com/prometheus/client_golang).


## Documentation

Find the latest documentation at https://docs.rs/prometheus


## Advanced

### Features

This library supports four features:

- `gen`: To generate protobuf client with the latest protobuf version instead of
  using the pre-generated client.

- `nightly`: Enable nightly only features.

- `process`: For collecting process info.

- `push`: Enable push support.


### Static Metric

When using a `MetricVec` with label values known at compile time
prometheus-static-metric reduces the overhead of retrieving the concrete
`Metric` from a `MetricVec`.

See [static-metric](./static-metric) directory for details.


## Thanks

- [brian-brazil](https://github.com/brian-brazil)
- [ccmtaylor](https://github.com/ccmtaylor)
- [kamalmarhubi](https://github.com/kamalmarhubi)
- [lucab](https://github.com/lucab)
- [koushiro](https://github.com/koushiro)
