// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;
extern crate prometheus_static_metric;

use prometheus::IntCounterVec;
use prometheus_static_metric::make_static_metric;

make_static_metric! {
    pub struct HttpRequestStatistics: IntCounter {
        "method" => {
            post,
            get,
            put,
            delete,
        },
        "version" => {
            http1: "HTTP/1",
            http2: "HTTP/2",
        },
        "product" => {
            foo,
            bar,
        },
    }
}

lazy_static! {
    pub static ref HTTP_COUNTER_VEC: IntCounterVec =
        register_int_counter_vec!(
            "http_requests",
            "Total number of HTTP requests.",
            &["product", "method", "version"]    // it doesn't matter for the label order
        ).unwrap();

    pub static ref HTTP_COUNTER: HttpRequestStatistics = HttpRequestStatistics
        ::from(&HTTP_COUNTER_VEC);
}

/// This example demonstrates the usage of:
/// 1. using alternative metric types (i.e. IntCounter)
/// 2. specifying different label order compared to the definition
/// 3. using non-identifiers as values

fn main() {
    HTTP_COUNTER.post.http1.foo.inc_by(4);
    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        4
    );

    // Note: You cannot specify values other than the definition in `get()` because
    // it is purely static.
    HTTP_COUNTER
        .try_get("delete")
        .unwrap()
        .try_get("HTTP/1")
        .unwrap()
        .foo
        .inc_by(7);
    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "delete", "HTTP/1"])
            .get(),
        7
    );
}
