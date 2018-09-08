// Copyright 2018 PingCAP, Inc.
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
