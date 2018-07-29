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

extern crate prometheus;

#[cfg(all(feature = "process", target_os = "linux"))]
fn main() {
    use std::thread;
    use std::time::Duration;

    use prometheus::{self, Encoder};

    // A default ProcessCollector is registered automatically.
    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();
    for _ in 0..5 {
        let metric_families = prometheus::gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        // Output to the standard output.
        println!("{}", String::from_utf8(buffer.clone()).unwrap());

        buffer.clear();
        thread::sleep(Duration::from_secs(1));
    }
}

#[cfg(any(not(feature = "process"), not(target_os = "linux")))]
fn main() {
    println!(
        r#"Please enable feature "process", try:
    cargo run --features="process" --example example_process_collector"#
    );
}
