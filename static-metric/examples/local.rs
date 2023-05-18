// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::cell::Cell;

use prometheus::*;

use once_cell::sync::Lazy;
use prometheus_static_metric::make_static_metric;

make_static_metric! {
    pub label_enum Methods {
        post,
        get,
        put,
        delete,
    }

    pub struct LocalHttpRequestStatistics: LocalIntCounter {
        "product" => {
            foo,
            bar,
        },
        "method" => Methods,
        "version" => {
            http1: "HTTP/1",
            http2: "HTTP/2",
        },
    }
}

static HTTP_COUNTER_VEC: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "http_requests_total",
        "Number of HTTP requests.",
        &["product", "method", "version"] // it doesn't matter for the label order
    )
    .unwrap()
});

thread_local! {
    static THREAD_LAST_TICK_TIME: Cell<u64> = Cell::new(timer::now_millis());

    pub static TLS_HTTP_COUNTER: LocalHttpRequestStatistics = LocalHttpRequestStatistics::from(&HTTP_COUNTER_VEC);
}

pub fn may_flush_metrics() {
    THREAD_LAST_TICK_TIME.with(|tls_last_tick| {
        let now = timer::now_millis();
        let last_tick = tls_last_tick.get();
        if now < last_tick + 1000 {
            return;
        }
        tls_last_tick.set(now);
        TLS_HTTP_COUNTER.with(|m| m.flush());
    });
}

/// This example demonstrates the usage of using static metrics with local metrics.

fn main() {
    TLS_HTTP_COUNTER.with(|m| m.foo.post.http1.inc());
    TLS_HTTP_COUNTER.with(|m| m.foo.post.http1.inc());
    TLS_HTTP_COUNTER.with(|m| m.foo.post.http1.inc());

    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        0
    );
    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/2"])
            .get(),
        0
    );

    may_flush_metrics();

    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        0
    );
    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/2"])
            .get(),
        0
    );

    ::std::thread::sleep(::std::time::Duration::from_secs(2));

    may_flush_metrics();

    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        3
    );
    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/2"])
            .get(),
        0
    );
}
