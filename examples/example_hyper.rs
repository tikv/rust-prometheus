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
extern crate hyper;
#[macro_use]
extern crate lazy_static;

use std::time::Instant;

use hyper::header::ContentType;
use hyper::server::{Server, Request, Response};
use hyper::mime::Mime;

use prometheus::encoder::{Encoder, TextEncoder};
use prometheus::{Counter, Opts, Gauge, Histogram, HistogramOpts};

lazy_static! {
    static ref HTTP_COUNTER: Counter = {
        let counter_opts =
            Opts::new("example_http_requests_total", "Total number of HTTP requests made.")
                .const_label("handler", "all");

        let counter = Counter::with_opts(counter_opts).unwrap();

        prometheus::register(Box::new(counter.clone())).unwrap();

        counter
    };

    static ref HTTP_BODY_GAUGE: Gauge = {
        let gauge_opts =
            Opts::new("example_http_response_size_bytes", "The HTTP response sizes in bytes.")
                .const_label("handler", "all");

        let gauge = Gauge::with_opts(gauge_opts).unwrap();

        prometheus::register(Box::new(gauge.clone())).unwrap();

        gauge
    };

    static ref HTTP_REQ_HISTOGRAM: Histogram = {
        let histogram_opts =
            HistogramOpts::new(
                "example_http_request_duration_microseconds",
                "The HTTP request latencies in microseconds.")
                .const_label("handler", "all");

        let histogram = Histogram::with_opts(histogram_opts).unwrap();

        prometheus::register(Box::new(histogram.clone())).unwrap();

        histogram
    };
}

fn main() {
    let encoder = TextEncoder::new();
    let addr = "127.0.0.1:9897";
    println!("check out {}!", addr);
    Server::http(addr)
        .unwrap()
        .handle(move |_: Request, mut res: Response| {
            HTTP_COUNTER.inc();
            let start = Instant::now();

            let metric_familys = prometheus::gather();
            let mut buffer = vec![];
            encoder.encode(&metric_familys, &mut buffer).unwrap();
            res.headers_mut()
                .set(ContentType(encoder.format_type().parse::<Mime>().unwrap()));
            res.send(&buffer).unwrap();

            let spend = (start.elapsed().subsec_nanos() as f64) / 1e6;
            HTTP_REQ_HISTOGRAM.observe(spend);
            HTTP_BODY_GAUGE.set(buffer.len() as f64);
        })
        .unwrap();
}
