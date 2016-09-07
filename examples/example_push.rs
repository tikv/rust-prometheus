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
#[macro_use]
extern crate lazy_static;

use std::thread;
use std::time;

use prometheus::{Histogram, Counter};

lazy_static! {
    static ref PUSH_COUNTER: Counter = register_counter!(
        opts!(
            "example_push_total",
            "Total number of prometheus client pushed."
        )
    ).unwrap();

    static ref PUSH_REQ_HISTOGRAM: Histogram = register_histogram!(
        histogram_opts!(
            "example_push_request_duration_seconds",
            "The push request latencies in seconds."
        )
    ).unwrap();
}

fn main() {
    println!("Pushing, please wait 10 seconds ...");

    for _ in 0..5 {
        thread::sleep(time::Duration::from_secs(2));
        PUSH_COUNTER.inc();
        let metric_familys = prometheus::gather();
        let _timer = PUSH_REQ_HISTOGRAM.start_timer(); // drop as observe
        prometheus::push_from_gather("example_push",
                                     labels!{"instance".to_owned() => "HAL-9000".to_owned(),},
                                     "127.0.0.1:9091",
                                     metric_familys);
    }
}
