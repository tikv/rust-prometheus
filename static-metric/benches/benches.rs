// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#![feature(test)]
#![feature(proc_macro)]

extern crate prometheus;
extern crate prometheus_static_metric;
extern crate test;

use prometheus::{IntCounter, IntCounterVec, Opts};
use prometheus_static_metric::make_static_metric;
use test::Bencher;

#[bench]
/// Single `IntCounter` performance.
fn bench_single_counter(b: &mut Bencher) {
    let counter = IntCounter::new("foo", "bar").unwrap();
    b.iter(|| counter.inc());
}

#[bench]
/// `IntCounterVec` performance.
fn bench_counter_vec(b: &mut Bencher) {
    let counter_vec = IntCounterVec::new(Opts::new("foo", "bar"), &["d1", "d2"]).unwrap();
    b.iter(|| counter_vec.with_label_values(&["foo", "bar"]).inc());
}

#[bench]
/// Manually implemented static metrics performance, metrics are placed outside a struct.
fn bench_static_metrics_handwrite_1(b: &mut Bencher) {
    let counter_vec = IntCounterVec::new(Opts::new("foo", "bar"), &["d1", "d2"]).unwrap();
    let counter = counter_vec.with_label_values(&["foo", "bar"]);
    b.iter(|| counter.inc());
}

#[bench]
/// Manually implemented static metrics performance, metrics are placed nested inside a struct.
fn bench_static_metrics_handwrite_2(b: &mut Bencher) {
    let counter_vec = IntCounterVec::new(Opts::new("foo", "bar"), &["d1", "d2"]).unwrap();
    struct StaticCounter1 {
        foo: StaticCounter1Field2,
    }
    struct StaticCounter1Field2 {
        bar: IntCounter,
    }
    let static_counter = StaticCounter1 {
        foo: StaticCounter1Field2 {
            bar: counter_vec.with_label_values(&["foo", "bar"]),
        },
    };
    b.iter(|| static_counter.foo.bar.inc());
}

make_static_metric! {
    struct StaticCounter2: IntCounter {
        "d1" => {
            foo,
        },
        "d2" => {
            bar,
        },
    }
}

#[bench]
/// macro implemented static metrics performance.
fn bench_static_metrics_macro(b: &mut Bencher) {
    let counter_vec = IntCounterVec::new(Opts::new("foo", "bar"), &["d1", "d2"]).unwrap();
    let static_counter = StaticCounter2::from(&counter_vec);
    b.iter(|| static_counter.foo.bar.inc());
}

#[bench]
/// macro implemented static metrics performance, with dynamic lookup.
fn bench_static_metrics_macro_with_lookup(b: &mut Bencher) {
    let counter_vec = IntCounterVec::new(Opts::new("foo", "bar"), &["d1", "d2"]).unwrap();
    let static_counter = StaticCounter2::from(&counter_vec);
    b.iter(|| static_counter.get("foo").get("bar").inc());
}

make_static_metric! {
    struct StaticCounter3: IntCounter {
        "d1" => { val1 },
        "d2" => { val2 },
        "d3" => { val3 },
        "d4" => { val4 },
        "d5" => { val5 },
        "d6" => { val6 },
        "d7" => { val7 },
        "d8" => { val8 },
        "d9" => { val9 },
        "d10" => { val10 },
    }
}

#[bench]
/// macro implemented static metrics performance, with a deep nesting level.
fn bench_static_metrics_macro_deep(b: &mut Bencher) {
    let counter_vec = IntCounterVec::new(
        Opts::new("foo", "bar"),
        &["d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "d10"],
    )
    .unwrap();
    let static_counter = StaticCounter3::from(&counter_vec);
    b.iter(|| {
        static_counter
            .val1
            .val2
            .val3
            .val4
            .val5
            .val6
            .val7
            .val8
            .val9
            .val10
            .inc()
    });
}
