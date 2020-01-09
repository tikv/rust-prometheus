// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[macro_use]
extern crate lazy_static;
extern crate coarsetime;
extern crate prometheus;
extern crate prometheus_static_metric;

use std::cell::Cell;

use coarsetime::Instant;
use prometheus::*;

#[allow(unused_imports)]
use prometheus::local::*;
use std::collections::HashMap;

#[allow(missing_copy_implementations)]
pub struct LocalHttpRequestStatistics {
    pub foo: LocalIntCounter,
    pub bar: LocalIntCounter,
    last_flush: Cell<Instant>,
}

impl LocalHttpRequestStatistics {
    pub fn from(m: &IntCounterVec) -> LocalHttpRequestStatistics {
        LocalHttpRequestStatistics {
            foo: m
                .with(&{
                    let mut coll = HashMap::new();
                    coll.insert("product", "foo");
                    coll
                })
                .local(),
            bar: m
                .with(&{
                    let mut coll = HashMap::new();
                    coll.insert("product", "bar");
                    coll
                })
                .local(),
            last_flush: Cell::new(Instant::now()),
        }
    }
    pub fn try_get(&self, value: &str) -> Option<&LocalIntCounter> {
        match value {
            "foo" => Some(&self.foo),
            "bar" => Some(&self.bar),
            _ => None,
        }
    }
    pub fn flush(&self) {
        self.foo.flush();
        self.bar.flush();
    }
}

impl ::prometheus::local::LocalMetric for LocalHttpRequestStatistics {
    fn flush(&self) {
        LocalHttpRequestStatistics::flush(self);
    }
}

lazy_static! {
    pub static ref HTTP_COUNTER_VEC: IntCounterVec =
        register_int_counter_vec!(
            "http_requests",
            "Total number of HTTP requests.",
            &["product"]    // it doesn't matter for the label order
        ).unwrap();
}

thread_local! {
    static THREAD_LAST_TICK_TIME: Cell<Instant> = Cell::new(Instant::now());

    pub static TLS_HTTP_COUNTER: LocalHttpRequestStatistics = LocalHttpRequestStatistics::from(&HTTP_COUNTER_VEC);
}

pub fn may_flush_metrics() {
    THREAD_LAST_TICK_TIME.with(|tls_last_tick| {
        let now = Instant::now();
        let last_tick = tls_last_tick.get();
        if now.duration_since(last_tick).as_f64() < 1.0 {
            return;
        }
        tls_last_tick.set(now);
        TLS_HTTP_COUNTER.with(|m| m.flush());
    });
}

/// This example demonstrates the usage of using static metrics with local metrics.

fn main() {
    TLS_HTTP_COUNTER.with(|m| m.foo.inc());
    TLS_HTTP_COUNTER.with(|m| m.foo.inc());
    TLS_HTTP_COUNTER.with(|m| m.foo.inc());

    assert_eq!(HTTP_COUNTER_VEC.with_label_values(&["foo"]).get(), 0);

    may_flush_metrics();

    assert_eq!(HTTP_COUNTER_VEC.with_label_values(&["foo"]).get(), 0);

    ::std::thread::sleep(::std::time::Duration::from_secs(2));

    may_flush_metrics();

    assert_eq!(HTTP_COUNTER_VEC.with_label_values(&["foo"]).get(), 3);
}
