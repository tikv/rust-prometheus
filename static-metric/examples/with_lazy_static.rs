// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use prometheus::CounterVec;

use lazy_static::lazy_static;
use prometheus::register_counter_vec;
use prometheus_static_metric::make_static_metric;

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
}

lazy_static! {
    pub static ref HTTP_COUNTER_VEC: CounterVec = register_counter_vec!(
        "http_requests_total",
        "Number of HTTP requests.",
        &["method", "product"]
    )
    .unwrap();
    pub static ref HTTP_COUNTER: HttpRequestStatistics =
        HttpRequestStatistics::from(&HTTP_COUNTER_VEC);
}

fn main() {
    HTTP_COUNTER.post.foo.inc();
    HTTP_COUNTER.delete.bar.inc_by(4.0);
    assert_eq!(HTTP_COUNTER.post.bar.get(), 0.0);
    assert_eq!(
        HTTP_COUNTER_VEC.with_label_values(&["post", "foo"]).get(),
        1.0
    );
    assert_eq!(
        HTTP_COUNTER_VEC.with_label_values(&["delete", "bar"]).get(),
        4.0
    );
}
