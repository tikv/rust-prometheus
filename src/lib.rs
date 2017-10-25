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

#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(feature = "nightly", feature(integer_atomics))]

#[macro_use]
extern crate quick_error;
extern crate protobuf;
extern crate fnv;
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "push")]
extern crate hyper;
#[cfg(any(feature = "nightly", feature = "push", feature = "process"))]
extern crate libc;
#[cfg(all(feature = "process", target_os = "linux"))]
extern crate procinfo;
#[macro_use]
extern crate cfg_if;

mod errors;
mod encoder;
#[macro_use]
mod macros;
mod metrics;
mod desc;
mod value;
mod counter;
mod gauge;
mod registry;
mod vec;
mod histogram;
#[cfg(feature = "push")]
mod push;
mod atomic64;

// Mods

/// Protocol buffers format of metrics.
#[path="../proto/metrics.rs"]
pub mod proto;
#[cfg(all(feature = "process", target_os = "linux"))]
pub mod process_collector;
pub mod local;

// Traits

pub use self::counter::{Counter, CounterVec};
pub use self::desc::Desc;
pub use self::encoder::{ProtobufEncoder, TextEncoder};
pub use self::encoder::Encoder;
pub use self::encoder::PROTOBUF_FORMAT;

// Constants
pub use self::encoder::TEXT_FORMAT;

// Structs
pub use self::errors::{Error, Result};
pub use self::gauge::{Gauge, GaugeVec};
pub use self::histogram::{Histogram, HistogramOpts, HistogramTimer, HistogramVec};
pub use self::histogram::{exponential_buckets, linear_buckets};
pub use self::histogram::DEFAULT_BUCKETS;
pub use self::metrics::Collector;
pub use self::metrics::Opts;
#[cfg(feature = "push")]
pub use self::push::{hostname_grouping_key, push_add_collector, push_add_metrics, push_collector,
                     push_metrics};

// Functions
pub use self::registry::{gather, register, unregister};
pub use self::registry::Registry;
pub use self::vec::MetricVec;
