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

#![feature(proc_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;
extern crate prometheus_static_metric;

use prometheus::CounterVec;
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
        "http_requests",
        "Total number of HTTP requests.",
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
