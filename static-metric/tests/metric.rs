// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

extern crate prometheus;
extern crate prometheus_static_metric;

use prometheus::core::Collector;
use prometheus::{Counter, CounterVec, Opts};
use prometheus_static_metric::make_static_metric;

make_static_metric! {
    pub label_enum Methods {
        post,
        get,
        put,
        delete,
    }

    pub label_enum MethodsWithName {
        post: "post_name",
        get: "get_name",
        put,
        delete,
    }

    pub struct SimpleCounterVec: Counter {
        "method" => Methods,
        "product" => {
            foo,
            bar,
        },
    }

    pub struct ComplexCounterVec: Counter {
        "method" => MethodsWithName,
        "product" => {
            foo,
            bar: "bar_name",
        },
    }
}

/// Helper method to get a label values of a `Counter`.
fn get_labels(counter: &Counter) -> Vec<String> {
    counter.collect()[0].get_metric()[0]
        .get_label()
        .into_iter()
        .map(|label| label.get_value().to_string())
        .collect()
}

#[test]
fn test_fields() {
    let vec = CounterVec::new(Opts::new("foo", "bar"), &["method", "product"]).unwrap();
    let metric = SimpleCounterVec::from(&vec);
    assert_eq!(get_labels(&metric.post.foo), vec!["post", "foo"]);
    assert_eq!(get_labels(&metric.put.bar), vec!["put", "bar"]);
}

#[test]
fn test_field_value() {
    let vec = CounterVec::new(Opts::new("foo", "bar"), &["method", "product"]).unwrap();
    let metric = ComplexCounterVec::from(&vec);
    assert_eq!(get_labels(&metric.post.foo), vec!["post_name", "foo"]);
    assert_eq!(get_labels(&metric.put.bar), vec!["put", "bar_name"]);
}

#[test]
fn test_label_order() {
    let vec = CounterVec::new(Opts::new("foo", "bar"), &["product", "method"]).unwrap();
    let metric = SimpleCounterVec::from(&vec);
    assert_eq!(get_labels(&metric.post.foo), vec!["post", "foo"]);
    assert_eq!(get_labels(&metric.put.bar), vec!["put", "bar"]);
}

#[test]
#[should_panic]
fn test_wrong_label_1() {
    let vec = CounterVec::new(Opts::new("foo", "bar"), &["method_foo", "product"]).unwrap();
    SimpleCounterVec::from(&vec);
}

#[test]
#[should_panic]
fn test_wrong_label_2() {
    let vec = CounterVec::new(Opts::new("foo", "bar"), &["method", "product_foo"]).unwrap();
    SimpleCounterVec::from(&vec);
}

#[test]
fn test_try_get() {
    let vec = CounterVec::new(Opts::new("foo", "bar"), &["method", "product"]).unwrap();
    let metric = SimpleCounterVec::from(&vec);
    assert_eq!(
        get_labels(&metric.try_get("get").unwrap().bar),
        vec!["get", "bar"]
    );
    assert_eq!(
        get_labels(&metric.get.try_get("bar").unwrap()),
        vec!["get", "bar"]
    );
    assert_eq!(
        get_labels(&metric.try_get("get").unwrap().try_get("bar").unwrap()),
        vec!["get", "bar"]
    );
    assert!(metric.try_get("get_foo").is_none());
    assert!(metric.try_get("get").unwrap().try_get("bar2").is_none());
}

#[test]
fn test_try_get_with_field_value() {
    let vec = CounterVec::new(Opts::new("foo", "bar"), &["method", "product"]).unwrap();
    let metric = ComplexCounterVec::from(&vec);
    assert_eq!(
        get_labels(&metric.try_get("get_name").unwrap().bar),
        vec!["get_name", "bar_name"]
    );
    assert_eq!(
        get_labels(&metric.get.try_get("bar_name").unwrap()),
        vec!["get_name", "bar_name"]
    );
    assert_eq!(
        get_labels(
            &metric
                .try_get("get_name")
                .unwrap()
                .try_get("bar_name")
                .unwrap()
        ),
        vec!["get_name", "bar_name"]
    );
    assert!(metric.try_get("get").is_none());
}

#[test]
fn test_get() {
    let vec = CounterVec::new(Opts::new("foo", "bar"), &["method", "product"]).unwrap();
    let metric = SimpleCounterVec::from(&vec);
    assert_eq!(
        get_labels(&metric.get(Methods::get).bar),
        vec!["get", "bar"]
    );
}

#[test]
fn test_get_with_field_value() {
    let vec = CounterVec::new(Opts::new("foo", "bar"), &["method", "product"]).unwrap();
    let metric = ComplexCounterVec::from(&vec);
    assert_eq!(
        get_labels(&metric.get(MethodsWithName::get).bar),
        vec!["get_name", "bar_name"]
    );
}
