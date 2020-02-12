// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/*!

Use metric enums to reuse possible values of a label.

*/
#[macro_use]
extern crate lazy_static;
extern crate coarsetime;
extern crate prometheus;
extern crate prometheus_static_metric;

use prometheus::*;
use prometheus_static_metric::make_auto_flush_static_metric;

make_auto_flush_static_metric! {


    pub label_enum FooBar {
        foo,
        bar,
    }

    pub label_enum Methods {
        post,
        get,
        put,
        delete,
    }

    pub struct Lhrs: LocalIntCounter {
        "product" => FooBar,
        "method" => Methods,
        "version" => {
            http1: "HTTP/1",
            http2: "HTTP/2",
        },
    }
}

lazy_static! {
pub static ref HTTP_COUNTER_VEC: IntCounterVec =
register_int_counter_vec ! (
"http_requests",
"Total number of HTTP requests.",
& ["product", "method", "version"]    // it doesn't matter for the label order
).unwrap();
}

thread_local! {
pub static TLS_HTTP_COUNTER_INNER: LhrsInner = LhrsInner::from(& HTTP_COUNTER_VEC);
}

lazy_static! {
    pub static ref TLS_HTTP_COUNTER: Lhrs = Lhrs::from(&TLS_HTTP_COUNTER_INNER);
}

fn main() {
    TLS_HTTP_COUNTER.foo.post.http1.inc();
    TLS_HTTP_COUNTER.foo.post.http1.inc();

    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        0
    );

    ::std::thread::sleep(::std::time::Duration::from_secs(2));

    TLS_HTTP_COUNTER.foo.post.http1.inc();
    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        3
    );
}
