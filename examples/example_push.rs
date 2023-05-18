// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#![cfg_attr(not(feature = "push"), allow(unused_imports, dead_code))]

use std::env;
use std::thread;
use std::time;

use getopts::Options;
use prometheus::{Counter, Histogram};

use once_cell::sync::Lazy;
use prometheus::{labels, register_counter, register_histogram};

static PUSH_COUNTER: Lazy<Counter> = Lazy::new(|| {
    register_counter!(
        "example_push_total",
        "Total number of prometheus client pushed."
    )
    .unwrap()
});
static PUSH_REQ_HISTOGRAM: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(
        "example_push_request_duration_seconds",
        "The push request latencies in seconds."
    )
    .unwrap()
});

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

    let address = matches
        .opt_str("A")
        .unwrap_or_else(|| "127.0.0.1:9091".to_owned());
    for _ in 0..5 {
        thread::sleep(time::Duration::from_secs(2));
        PUSH_COUNTER.inc();
        let metric_families = prometheus::gather();
        let _timer = PUSH_REQ_HISTOGRAM.start_timer(); // drop as observe
        prometheus::push_metrics(
            "example_push",
            labels! {"instance".to_owned() => "HAL-9000".to_owned(),},
            &address,
            metric_families,
            Some(prometheus::BasicAuthentication {
                username: "user".to_owned(),
                password: "pass".to_owned(),
            }),
        )
        .unwrap();
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
