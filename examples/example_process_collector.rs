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

#[macro_use]
extern crate prometheus;

#[cfg(feature = "process")]
mod process {
    use std::thread;
    use std::time::Duration;

    use prometheus::{self, process_collector, Encoder};

    pub fn demo() {
        // A default ProcessCollector is registered automatically.

        let mut buffer = Vec::new();
        let encoder = prometheus::TextEncoder::new();
        for _ in 0..5 {
            let metric_familys = prometheus::gather();
            encoder.encode(&metric_familys, &mut buffer).unwrap();

            // Output to the standard output.
            println!("{}", String::from_utf8(buffer.clone()).unwrap());

            buffer.clear();
            thread::sleep(Duration::from_secs(1));
        }
    }
}

#[cfg(not(feature = "process"))]
mod process {
    pub fn demo() {
        println!("Please enable feature \"process\", then try:\n\tcargo run \
                  --features=\"process\" --example example_process_collector");
    }
}

fn main() {
    process::demo();
}
