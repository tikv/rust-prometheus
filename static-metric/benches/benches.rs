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
    pub struct StaticCounter2: IntCounter {
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
