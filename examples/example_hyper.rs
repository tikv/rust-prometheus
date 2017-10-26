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
extern crate hyper;
#[macro_use]
extern crate lazy_static;

use hyper::header::ContentType;
use hyper::mime::Mime;
use hyper::server::{Request, Response, Server};

use prometheus::{Counter, Encoder, Gauge, HistogramVec, TextEncoder};

lazy_static! {
    static ref HTTP_COUNTER: Counter = register_counter!(
        opts!(
            "example_http_requests_total",
            "Total number of HTTP requests made.",
            labels!{"handler" => "all",}
        )
    ).unwrap();

    static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(
        opts!(
            "example_http_response_size_bytes",
            "The HTTP response sizes in bytes.",
            labels!{"handler" => "all",}
        )
    ).unwrap();

    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "example_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    ).unwrap();
}

fn main() {
    let encoder = TextEncoder::new();
    let addr = "127.0.0.1:9898";
    println!("listening addr {:?}", addr);
    Server::http(addr)
        .unwrap()
        .handle(move |_: Request, mut res: Response| {
            HTTP_COUNTER.inc();
            let timer = HTTP_REQ_HISTOGRAM.with_label_values(&["all"]).start_timer();

            let metric_familys = prometheus::gather();
            let mut buffer = vec![];
            encoder.encode(&metric_familys, &mut buffer).unwrap();
            res.headers_mut()
                .set(ContentType(encoder.format_type().parse::<Mime>().unwrap()));
            res.send(&buffer).unwrap();

            timer.observe_duration();
            HTTP_BODY_GAUGE.set(buffer.len() as f64);
        })
        .unwrap();
}
