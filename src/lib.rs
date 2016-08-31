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

#[macro_use]
extern crate quick_error;
extern crate protobuf;
extern crate fnv;
#[macro_use]
extern crate lazy_static;

pub mod proto;
/// Error types of prometheus.
pub mod errors;
/// Encoders of prometheus.
pub mod encoder;

mod macros;
mod metrics;
mod desc;
// TODO: remove dead_code later.
#[allow(dead_code)]
mod value;
mod counter;
mod gauge;
mod registry;
#[allow(dead_code)]
mod vec;
mod histogram;

// Structs
pub use self::desc::Desc;
pub use self::registry::Registry;
pub use self::metrics::{Collector, Opts};
pub use self::counter::{Counter, CounterVec};
pub use self::gauge::{Gauge, GaugeVec};
pub use self::histogram::{Histogram, HistogramVec, HistogramOpts};

// functions
pub use self::registry::{gather, register, unregister};
pub use self::histogram::{linear_buckets, exponential_buckets};

// Constants
pub use self::histogram::DEFAULT_BUCKETS;
