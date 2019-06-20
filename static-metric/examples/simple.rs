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

extern crate prometheus;
extern crate prometheus_static_metric;

use prometheus::{CounterVec, Opts};
use prometheus_static_metric::make_static_metric;

make_static_metric! {
    pub struct MyStaticCounterVec: Counter {
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

fn main() {
    let vec = CounterVec::new(Opts::new("foo", "bar"), &["method", "product"]).unwrap();
    let static_counter_vec = MyStaticCounterVec::from(&vec);

    static_counter_vec.post.foo.inc();
    static_counter_vec.delete.bar.inc_by(4.0);
    assert_eq!(static_counter_vec.post.bar.get(), 0.0);
    assert_eq!(vec.with_label_values(&["post", "foo"]).get(), 1.0);
    assert_eq!(vec.with_label_values(&["delete", "bar"]).get(), 4.0);
}
