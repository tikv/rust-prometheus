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

/*!
The Rust client library for [Prometheus](https://prometheus.io/).
*/

#![cfg_attr(all(test, bench), feature(test))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(not(feature = "dev"), allow(unknown_lints))]
#![cfg_attr(feature = "dev", allow(needless_pass_by_value))]
#![cfg_attr(feature = "dev", allow(new_without_default_derive))]
#![cfg_attr(feature = "nightly", feature(integer_atomics))]

#[macro_use]
extern crate cfg_if;
extern crate fnv;
#[cfg(feature = "push")]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[cfg(any(feature = "nightly", feature = "push", feature = "process"))]
extern crate libc;
#[cfg(all(feature = "process", target_os = "linux"))]
extern crate procinfo;
extern crate protobuf;
#[macro_use]
extern crate quick_error;
extern crate spin;
#[cfg(all(test, bench))]
extern crate test;

mod encoder;
mod errors;
#[macro_use]
mod macros;
mod atomic64;
mod counter;
mod desc;
mod gauge;
mod histogram;
mod metrics;
#[cfg(feature = "push")]
mod push;
mod registry;
mod value;
mod vec;

#[cfg(all(feature = "process", target_os = "linux"))]
pub mod process_collector;
/// Protocol buffers format of metrics.
#[path = "../proto/metrics.rs"]
pub mod proto;

pub mod local {
    /*!

    Unsync local metrics, provides better performance.

    */

    // TODO: define a trait and implement the trait for metrics.

    pub use super::counter::{LocalCounter, LocalCounterVec, LocalIntCounter, LocalIntCounterVec};
    pub use super::histogram::{LocalHistogram, LocalHistogramTimer, LocalHistogramVec};
}

pub mod core {
    /*!

    Core traits and types.

    */

    pub use super::atomic64::*;
    pub use super::counter::{GenericCounter, GenericCounterVec, GenericLocalCounter,
                             GenericLocalCounterVec};
    pub use super::desc::{Desc, Describer};
    pub use super::gauge::{GenericGauge, GenericGaugeVec};
    pub use super::metrics::{Collector, Metric, Opts};
    pub use super::vec::{MetricVec, MetricVecBuilder};
}

pub use self::counter::{Counter, CounterVec, IntCounter, IntCounterVec};
pub use self::encoder::Encoder;
pub use self::encoder::{PROTOBUF_FORMAT, TEXT_FORMAT};
pub use self::encoder::{ProtobufEncoder, TextEncoder};
pub use self::errors::{Error, Result};
pub use self::gauge::{Gauge, GaugeVec, IntGauge, IntGaugeVec};
pub use self::histogram::DEFAULT_BUCKETS;
pub use self::histogram::{Histogram, HistogramOpts, HistogramTimer, HistogramVec};
pub use self::histogram::{exponential_buckets, linear_buckets};
pub use self::metrics::Opts;
#[cfg(feature = "push")]
pub use self::push::{hostname_grouping_key, push_add_collector, push_add_metrics, push_collector,
                     push_metrics};
pub use self::registry::Registry;
pub use self::registry::{gather, register, unregister};
