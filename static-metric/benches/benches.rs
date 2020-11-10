// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

extern crate criterion;
extern crate prometheus;
extern crate prometheus_static_metric;

use criterion::{criterion_group, criterion_main, Criterion};
use prometheus::{IntCounter, IntCounterVec, Opts};
use prometheus_static_metric::make_static_metric;

/// Single `IntCounter` performance.
fn bench_single_counter(c: &mut Criterion) {
    let counter = IntCounter::new("foo", "bar").unwrap();
    c.bench_function("bench_single_counter", |b| {
        b.iter(|| counter.inc());
    });
}

/// `IntCounterVec` performance.
fn bench_counter_vec(c: &mut Criterion) {
    let counter_vec = IntCounterVec::new(Opts::new("foo", "bar"), &["d1", "d2"]).unwrap();
    c.bench_function("bench_counter_vec", |b| {
        b.iter(|| counter_vec.with_label_values(&["foo", "bar"]).inc());
    });
}

/// Manually implemented static metrics performance, metrics are placed outside a struct.
fn bench_static_metrics_handwrite_1(c: &mut Criterion) {
    let counter_vec = IntCounterVec::new(Opts::new("foo", "bar"), &["d1", "d2"]).unwrap();
    let counter = counter_vec.with_label_values(&["foo", "bar"]);
    c.bench_function("bench_static_metrics_handwrite_1", |b| {
        b.iter(|| counter.inc());
    });
}

/// Manually implemented static metrics performance, metrics are placed nested inside a struct.
fn bench_static_metrics_handwrite_2(c: &mut Criterion) {
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
    c.bench_function("bench_static_metrics_handwrite_2", |b| {
        b.iter(|| static_counter.foo.bar.inc());
    });
}

make_static_metric! {
    label_enum D1 {
        foo,
    }

    label_enum D2 {
        bar,
    }

    struct StaticCounter2: IntCounter {
        "d1" => D1,
        "d2" => D2,
    }
}

/// macro implemented static metrics performance.
fn bench_static_metrics_macro(c: &mut Criterion) {
    let counter_vec = IntCounterVec::new(Opts::new("foo", "bar"), &["d1", "d2"]).unwrap();
    let static_counter = StaticCounter2::from(&counter_vec);
    c.bench_function("bench_static_metrics_macro", |b| {
        b.iter(|| static_counter.foo.bar.inc());
    });
}

/// macro implemented static metrics performance, with dynamic lookup.
fn bench_static_metrics_macro_with_lookup(c: &mut Criterion) {
    let counter_vec = IntCounterVec::new(Opts::new("foo", "bar"), &["d1", "d2"]).unwrap();
    let static_counter = StaticCounter2::from(&counter_vec);
    c.bench_function("bench_static_metrics_macro_with_lookup", |b| {
        b.iter(|| static_counter.get(D1::foo).get(D2::bar).inc());
    });
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

/// macro implemented static metrics performance, with a deep nesting level.
fn bench_static_metrics_macro_deep(c: &mut Criterion) {
    let counter_vec = IntCounterVec::new(
        Opts::new("foo", "bar"),
        &["d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "d10"],
    )
    .unwrap();
    let static_counter = StaticCounter3::from(&counter_vec);
    c.bench_function("bench_static_metrics_macro_deep", |b| {
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
    });
}

criterion_group!(
    benches,
    bench_counter_vec,
    bench_single_counter,
    bench_static_metrics_handwrite_1,
    bench_static_metrics_handwrite_2,
    bench_static_metrics_macro,
    bench_static_metrics_macro_deep,
    bench_static_metrics_macro_with_lookup,
);
criterion_main!(benches);
