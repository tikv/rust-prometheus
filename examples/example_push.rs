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

#![cfg_attr(not(feature = "push"), allow(unused_imports, dead_code))]

extern crate getopts;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

use std::env;
use std::thread;
use std::time;

use getopts::Options;
use prometheus::{Counter, Histogram};

lazy_static! {
    static ref PUSH_COUNTER: Counter = register_counter!(
        "example_push_total",
        "Total number of prometheus client pushed."
    ).unwrap();
    static ref PUSH_REQ_HISTOGRAM: Histogram = register_histogram!(
        "example_push_request_duration_seconds",
        "The push request latencies in seconds."
    ).unwrap();
}

#[cfg(feature = "push")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "A",
        "addr",
        "prometheus pushgateway address",
        "default is 127.0.0.1:9091",
    );
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args).unwrap();
    if matches.opt_present("h") || !matches.opt_present("A") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return;
    }
    println!("Pushing, please start Pushgateway first.");

    let address = matches.opt_str("A").unwrap_or("127.0.0.1:9091".to_owned());
    for _ in 0..5 {
        thread::sleep(time::Duration::from_secs(2));
        PUSH_COUNTER.inc();
        let metric_familys = prometheus::gather();
        let _timer = PUSH_REQ_HISTOGRAM.start_timer(); // drop as observe
        prometheus::push_metrics(
            "example_push",
            labels!{"instance".to_owned() => "HAL-9000".to_owned(),},
            &address,
            metric_familys,
            Some(prometheus::BasicAuthentication {
                username: "user".to_owned(),
                password: "pass".to_owned(),
            }),
        ).unwrap();
    }

    println!("Okay, please check the Pushgateway.");
}

#[cfg(not(feature = "push"))]
fn main() {
    println!(
        r#"Please enable feature "push", try:
    cargo run --features="push" --example example_push"#
    );
}
