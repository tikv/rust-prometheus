// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/*!

You can integrate static metric with the `register_xxx!` macro,
by using the `register_static_xxx!` macro provided by this crate.

*/

#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;
extern crate prometheus_static_metric;

use prometheus::exponential_buckets;
use prometheus_static_metric::*;

make_static_metric! {
    pub struct HttpRequestStatistics: Counter {
        "method" => {
            post,
            get,
            put,
            delete,
        },
        "product" => {
            foo,
            bar,
        },
    }
    pub struct HttpRequestDuration: Histogram {
        "method" => {
            post,
            get,
            put,
            delete,
        }
    }
}

lazy_static! {
    pub static ref HTTP_COUNTER: HttpRequestStatistics = register_static_counter_vec!(
        HttpRequestStatistics,
        "http_requests",
        "Total number of HTTP requests.",
        &["method", "product"]
    )
    .unwrap();
    pub static ref HTTP_DURATION: HttpRequestDuration = register_static_histogram_vec!(
        HttpRequestDuration,
        "http_request_duration",
        "Duration of each HTTP request.",
        &["method"],
        exponential_buckets(0.0005, 2.0, 20).unwrap()
    )
    .unwrap();
}

fn main() {
    HTTP_COUNTER.post.foo.inc();
    HTTP_COUNTER.delete.bar.inc_by(4.0);
    assert_eq!(HTTP_COUNTER.post.bar.get(), 0.0);
    assert_eq!(HTTP_COUNTER.delete.bar.get(), 4.0);

    HTTP_DURATION.post.observe(0.5);
    HTTP_DURATION.post.observe(1.0);
}
