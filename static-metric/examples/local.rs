// Copyright 2018 PingCAP, Inc.
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
extern crate lazy_static;
extern crate prometheus;
extern crate prometheus_static_metric;
extern crate coarsetime;

use coarsetime::Instant;
use std::cell::{RefCell, Cell};

use prometheus::*;
use prometheus_static_metric::make_static_metric;

make_static_metric! {
    pub struct LocalHttpRequestStatistics: LocalIntCounter {
        "product" => {
            foo,
            bar,
        },
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
    }
}

lazy_static! {
    pub static ref HTTP_COUNTER_VEC: IntCounterVec =
        register_int_counter_vec!(
            "http_requests",
            "Total number of HTTP requests.",
            &["product", "method", "version"]    // it doesn't matter for the label order
        ).unwrap();
}

thread_local! {
    static THREAD_LAST_TICK_TIME: Cell<Instant> = Cell::new(Instant::now());

    pub static TLS_HTTP_COUNTER: RefCell<LocalHttpRequestStatistics> = RefCell::new(
        LocalHttpRequestStatistics::from(&HTTP_COUNTER_VEC)
    );
}

pub fn may_flush_metrics() {
    THREAD_LAST_TICK_TIME.with(|tls_last_tick| {
        let now = Instant::now();
        let last_tick = tls_last_tick.get();
        if now.duration_since(last_tick).as_f64() < 1.0 {
            return;
        }
        tls_last_tick.set(now);
        TLS_HTTP_COUNTER.with(|m| m.borrow_mut().flush());
    });
}


/// This example demonstrates the usage of using static metrics with local metrics.

fn main() {
    TLS_HTTP_COUNTER.with(|m| m.borrow_mut().foo.post.http1.inc());
    TLS_HTTP_COUNTER.with(|m| m.borrow_mut().foo.post.http1.inc());
    TLS_HTTP_COUNTER.with(|m| m.borrow_mut().foo.post.http1.inc());

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
