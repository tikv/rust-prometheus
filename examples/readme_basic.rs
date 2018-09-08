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
