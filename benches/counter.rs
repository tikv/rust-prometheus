// Copyright 2016 PingCAP, Inc.
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

extern crate prometheus;
extern crate test;

use std::collections::HashMap;

use prometheus::{Counter, CounterVec, IntCounter, Opts};
use test::Bencher;

#[bench]
fn bench_counter_with_label_values(b: &mut Bencher) {
    let counter = CounterVec::new(
        Opts::new("benchmark_counter", "A counter to benchmark it."),
        &["one", "two", "three"],
    )
    .unwrap();
    b.iter(|| counter.with_label_values(&["eins", "zwei", "drei"]).inc())
}

#[bench]
fn bench_counter_with_mapped_labels(b: &mut Bencher) {
    let counter = CounterVec::new(
        Opts::new("benchmark_counter", "A counter to benchmark it."),
        &["one", "two", "three"],
    )
    .unwrap();

    b.iter(|| {
        let mut labels = HashMap::with_capacity(3);
        labels.insert("two", "zwei");
        labels.insert("one", "eins");
        labels.insert("three", "drei");
        counter.with(&labels).inc();
    })
}

#[bench]
fn bench_counter_with_prepared_mapped_labels(b: &mut Bencher) {
    let counter = CounterVec::new(
        Opts::new("benchmark_counter", "A counter to benchmark it."),
        &["one", "two", "three"],
    )
    .unwrap();

    let mut labels = HashMap::with_capacity(3);
    labels.insert("two", "zwei");
    labels.insert("one", "eins");
    labels.insert("three", "drei");

    b.iter(|| {
        counter.with(&labels).inc();
    })
}

#[bench]
fn bench_counter_no_labels(b: &mut Bencher) {
    let counter = Counter::new("benchmark_counter", "A counter to benchmark.").unwrap();
    b.iter(|| counter.inc())
}

#[bench]
fn bench_int_counter_no_labels(b: &mut Bencher) {
    let counter = IntCounter::new("benchmark_int_counter", "A int_counter to benchmark.").unwrap();
    b.iter(|| counter.inc())
}
