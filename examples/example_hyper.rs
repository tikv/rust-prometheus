// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

use hyper::{header::CONTENT_TYPE, rt::Future, service::service_fn_ok, Body, Response, Server};
use prometheus::{Counter, Encoder, Gauge, HistogramVec, TextEncoder};

lazy_static! {
    static ref HTTP_COUNTER: Counter = register_counter!(opts!(
        "example_http_requests_total",
        "Total number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(opts!(
        "example_http_response_size_bytes",
        "The HTTP response sizes in bytes.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "example_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    )
    .unwrap();
}

fn main() {
    let addr = ([127, 0, 0, 1], 9898).into();
    println!("Listening address: {:?}", addr);

    let new_service = || {
        let encoder = TextEncoder::new();
        service_fn_ok(move |_request| {
            HTTP_COUNTER.inc();
            let timer = HTTP_REQ_HISTOGRAM.with_label_values(&["all"]).start_timer();

            let metric_families = prometheus::gather();
            let mut buffer = vec![];
            encoder.encode(&metric_families, &mut buffer).unwrap();
            HTTP_BODY_GAUGE.set(buffer.len() as f64);

            let response = Response::builder()
                .status(200)
                .header(CONTENT_TYPE, encoder.format_type())
                .body(Body::from(buffer))
                .unwrap();

            timer.observe_duration();

            response
        })
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("Server error: {}", e));

    hyper::rt::run(server);
}
