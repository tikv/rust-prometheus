// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/*!

You can integrate static metric with the `register_xxx!` macro,
by using the `register_static_xxx!` macro provided by this crate.

*/

use prometheus::exponential_buckets;

use once_cell::sync::Lazy;
use prometheus::{register_counter_vec, register_histogram_vec};
use prometheus_static_metric::{
    make_static_metric, register_static_counter_vec, register_static_histogram_vec,
};

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

static HTTP_COUNTER: Lazy<HttpRequestStatistics> = Lazy::new(|| {
    register_static_counter_vec!(
        HttpRequestStatistics,
        "http_requests_total",
        "Number of HTTP requests.",
        &["method", "product"]
    )
    .unwrap()
});
static HTTP_DURATION: Lazy<HttpRequestDuration> = Lazy::new(|| {
    register_static_histogram_vec!(
        HttpRequestDuration,
        "http_request_duration",
        "Duration of each HTTP request.",
        &["method"],
        exponential_buckets(0.0005, 2.0, 20).unwrap()
    )
    .unwrap()
});

fn main() {
    HTTP_COUNTER.post.foo.inc();
    HTTP_COUNTER.delete.bar.inc_by(4.0);
    assert_eq!(HTTP_COUNTER.post.bar.get(), 0.0);
    assert_eq!(HTTP_COUNTER.delete.bar.get(), 4.0);

    HTTP_DURATION.post.observe(0.5);
    HTTP_DURATION.post.observe(1.0);
}
