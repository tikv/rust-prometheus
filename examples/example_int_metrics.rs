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

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

use prometheus::{IntCounter, IntCounterVec, IntGauge, IntGaugeVec};

lazy_static! {
    static ref A_INT_COUNTER: IntCounter =
        register_int_counter!("A_int_counter", "foobar").unwrap();
    static ref A_INT_COUNTER_VEC: IntCounterVec =
        register_int_counter_vec!("A_int_counter_vec", "foobar", &["a", "b"]).unwrap();
    static ref A_INT_GAUGE: IntGauge = register_int_gauge!("A_int_gauge", "foobar").unwrap();
    static ref A_INT_GAUGE_VEC: IntGaugeVec =
        register_int_gauge_vec!("A_int_gauge_vec", "foobar", &["a", "b"]).unwrap();
}

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
