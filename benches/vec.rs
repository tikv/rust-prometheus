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

use prometheus::{Opts, CounterVec};

fn rand_string() -> String {
    rand::thread_rng().gen_ascii_chars().take(8).collect()
}

macro_rules! batch_counter_with_label_values {
    ($name:ident, $size: expr, $count: expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let counter = CounterVec::new(Opts::new("benchmark_counter",
                                                    "A counter to benchmark it."),
                                        &["one"])
                .unwrap();

            b.iter(|| {
                let mut map = HashMap::with_capacity($size);

                for _ in 0..$size {
                    map.insert(rand_string(), 0);
                }

                for val in map.values_mut() {
                    for _ in 0..$count {
                        *val = *val + 1;
                    }
                }

                for (key, val) in map {
                    counter.with_label_values(&[&key]).inc_by(val as f64).unwrap();
                }
            })
        }
    }
}

batch_counter_with_label_values!{bench_batch_counter_with_label_values_2, 2, 2}
batch_counter_with_label_values!{bench_batch_counter_with_label_values_4, 2, 4}
batch_counter_with_label_values!{bench_batch_counter_with_label_values_8, 2, 8}
batch_counter_with_label_values!{bench_batch_counter_with_label_values_16, 2, 16}

macro_rules! no_batch_counter_with_label_values {
    ($name:ident, $size: expr, $count: expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let counter = CounterVec::new(Opts::new("benchmark_counter",
                                                    "A counter to benchmark it."),
                                        &["one"])
                .unwrap();

            let mut map = Vec::new();
            for _ in 0..$size {
                map.push((rand_string(), $count));
            }

            b.iter(|| {
                for &(ref key, val) in &map {
                    for _ in 0..val {
                        counter.with_label_values(&[key]).inc();
                    }
                }
            })
        }
    }
}

no_batch_counter_with_label_values!{bench_no_batch_counter_with_label_values_2, 2, 2}
no_batch_counter_with_label_values!{bench_no_batch_counter_with_label_values_4, 2, 4}
no_batch_counter_with_label_values!{bench_no_batch_counter_with_label_values_8, 2, 8}
no_batch_counter_with_label_values!{bench_no_batch_counter_with_label_values_16, 2, 16}
