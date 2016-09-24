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

use std::collections::HashMap;

use test::Bencher;
use rand::{self, Rng};

use prometheus::{Opts, Counter, CounterVec};

fn rand_string() -> String {
    rand::thread_rng().gen_ascii_chars().take(8).collect()
}

macro_rules! counter_with_label_values {
    ($name:ident, $len: expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let counter = CounterVec::new(Opts::new("benchmark_counter",
                                                    "A counter to benchmark it."),
                                        &["one"])
                .unwrap();

            let mut values_vec = Vec::new();
            for _ in 0..$len {
                values_vec.push(rand_string());
            }

            for v in &values_vec {
                counter.with_label_values(&[v]);
            }

            let ref target = values_vec[values_vec.len() / 2];

            b.iter(|| {
                counter.with_label_values(&[&target]);
            })
        }
    }
}

counter_with_label_values!{bench_counter_with_label_values_2, 2}
counter_with_label_values!{bench_counter_with_label_values_4, 4}
counter_with_label_values!{bench_counter_with_label_values_8, 8}
#[cfg(not(feature = "nightly"))]
counter_with_label_values!{bench_counter_with_label_values_16, 16}
#[cfg(not(feature = "nightly"))]
counter_with_label_values!{bench_counter_with_label_values_32, 32}
#[cfg(not(feature = "nightly"))]
counter_with_label_values!{bench_counter_with_label_values_64, 64}

#[bench]
fn bench_counter_with_label_values(b: &mut Bencher) {
    let counter = CounterVec::new(Opts::new("benchmark_counter", "A counter to benchmark it."),
                                  &["one", "two", "three"])
        .unwrap();
    b.iter(|| counter.with_label_values(&["eins", "zwei", "drei"]).inc())
}

#[bench]
fn bench_counter_with_mapped_labels(b: &mut Bencher) {
    let counter = CounterVec::new(Opts::new("benchmark_counter", "A counter to benchmark it."),
                                  &["one", "two", "three"])
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
    let counter = CounterVec::new(Opts::new("benchmark_counter", "A counter to benchmark it."),
                                  &["one", "two", "three"])
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
    let counter = Counter::new("benchmark_counter", "A counter to benchmark it.").unwrap();
    b.iter(|| counter.inc())
}
