// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/*!

Use metric enums to reuse possible values of a label.

*/

extern crate prometheus;
extern crate prometheus_static_metric;

use prometheus::{CounterVec, IntCounterVec, Opts};
use prometheus_static_metric::make_static_metric;

make_static_metric! {
    pub label_enum Methods {
        post,
        get,
        put,
        delete,
    }

    pub label_enum Products {
        foo,
        bar,
    }

    pub struct MyStaticCounterVec: Counter {
        "method" => Methods,
        "product" => Products,
    }

    pub struct MyAnotherStaticCounterVec: IntCounter {
        "error" => {
            error_1,
            error_2,
        },
        "error_method" => Methods,
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

    // metric enums will expose an enum for type-safe `get()`.
    static_counter_vec.get(Methods::post).foo.inc();
    assert_eq!(static_counter_vec.post.foo.get(), 2.0);

    let vec = IntCounterVec::new(Opts::new("foo", "bar"), &["error", "error_method"]).unwrap();
    let static_counter_vec = MyAnotherStaticCounterVec::from(&vec);

    static_counter_vec.error_1.post.inc();
    static_counter_vec.error_2.delete.inc_by(4);
    assert_eq!(static_counter_vec.error_1.delete.get(), 0);
    assert_eq!(static_counter_vec.error_1.post.get(), 1);
    assert_eq!(vec.with_label_values(&["error_2", "delete"]).get(), 4);
}
