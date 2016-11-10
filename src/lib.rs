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

#![cfg_attr(feature="dev", feature(plugin))]
#![cfg_attr(feature="dev", plugin(clippy))]
#![cfg_attr(feature="nightly", feature(integer_atomics))]

#[macro_use]
extern crate quick_error;
extern crate protobuf;
extern crate fnv;
#[macro_use]
extern crate lazy_static;
#[cfg(feature="push")]
extern crate hyper;
#[cfg(feature="push")]
extern crate libc;

mod errors;
mod encoder;
#[macro_use]
mod macros;
mod metrics;
mod desc;
// TODO: remove dead_code later.
#[allow(dead_code)]
mod value;
mod counter;
mod gauge;
#[allow(dead_code)]
mod registry;
#[allow(dead_code)]
mod vec;
mod histogram;
#[cfg(feature="push")]
mod push;
mod atomic64;

// Mods

/// Protocol buffers format of metrics.
pub mod proto;

// Traits
pub use self::encoder::Encoder;
pub use self::metrics::Collector;

// Structs
pub use self::errors::{Result, Error};
pub use self::encoder::TextEncoder;
pub use self::desc::Desc;
pub use self::registry::Registry;
pub use self::metrics::Opts;
pub use self::counter::{Counter, CounterVec};
pub use self::gauge::{Gauge, GaugeVec};
pub use self::histogram::{Histogram, HistogramVec, HistogramOpts, HistogramTimer};

// Functions
pub use self::registry::{gather, register, unregister};
pub use self::histogram::{linear_buckets, exponential_buckets};
#[cfg(feature="push")]
pub use self::push::{push_metrics, push_add_metrics, push_collector, push_add_collector,
                     hostname_grouping_key};

// Constants
pub use self::encoder::TEXT_FORMAT;
pub use self::histogram::DEFAULT_BUCKETS;
