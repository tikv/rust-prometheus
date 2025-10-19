// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::sync::LazyLock;

use prometheus::{IntCounter, IntCounterVec, IntGauge, IntGaugeVec};

use prometheus::{
    register_int_counter, register_int_counter_vec, register_int_gauge, register_int_gauge_vec,
};

static A_INT_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| register_int_counter!("A_int_counter", "foobar").unwrap());
static A_INT_COUNTER_VEC: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!("A_int_counter_vec", "foobar", &["a", "b"]).unwrap()
});
static A_INT_GAUGE: LazyLock<IntGauge> =
    LazyLock::new(|| register_int_gauge!("A_int_gauge", "foobar").unwrap());
static A_INT_GAUGE_VEC: LazyLock<IntGaugeVec> =
    LazyLock::new(|| register_int_gauge_vec!("A_int_gauge_vec", "foobar", &["a", "b"]).unwrap());

fn main() {
    A_INT_COUNTER.inc();
    A_INT_COUNTER.inc_by(10);
    assert_eq!(A_INT_COUNTER.get(), 11);

    A_INT_COUNTER_VEC.with_label_values(&["a", "b"]).inc_by(5);
    assert_eq!(A_INT_COUNTER_VEC.with_label_values(&["a", "b"]).get(), 5);

    A_INT_COUNTER_VEC.with_label_values(&["c", "d"]).inc();
    assert_eq!(A_INT_COUNTER_VEC.with_label_values(&["c", "d"]).get(), 1);

    A_INT_GAUGE.set(5);
    assert_eq!(A_INT_GAUGE.get(), 5);
    A_INT_GAUGE.dec();
    assert_eq!(A_INT_GAUGE.get(), 4);
    A_INT_GAUGE.add(2);
    assert_eq!(A_INT_GAUGE.get(), 6);

    A_INT_GAUGE_VEC.with_label_values(&["a", "b"]).set(10);
    A_INT_GAUGE_VEC.with_label_values(&["a", "b"]).dec();
    A_INT_GAUGE_VEC.with_label_values(&["a", "b"]).sub(2);
    assert_eq!(A_INT_GAUGE_VEC.with_label_values(&["a", "b"]).get(), 7);
}
