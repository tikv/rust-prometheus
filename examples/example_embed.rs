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

extern crate prometheus;

use std::thread;
use std::time::Duration;

use prometheus::encoder::{TextEncoder, Encoder};
use prometheus::{Counter, Opts, Registry, CounterVec};

fn main() {
    let opts = Opts::new("test", "test help").const_label("a", "1").const_label("b", "2");
    let counter = Counter::with_opts(opts).unwrap();
    let opts =
        Opts::new("test_vec", "test vector help").const_label("a", "1").const_label("b", "2");
    let counter_vec = CounterVec::new(opts, &["c", "d"]).unwrap();

    let r = Registry::new();
    r.register(Box::new(counter.clone())).unwrap();
    r.register(Box::new(counter_vec.clone())).unwrap();

    counter.inc();
    assert_eq!(counter.get() as u64, 1);
    counter.inc_by(42.0).unwrap();
    assert_eq!(counter.get() as u64, 43);

    counter_vec.with_label_values(&["3", "4"]).inc();
    assert_eq!(counter_vec.with_label_values(&["3", "4"]).get() as u64, 1);

    counter_vec.with_label_values(&["3", "4"]).inc_by(42.0).unwrap();
    assert_eq!(counter_vec.with_label_values(&["3", "4"]).get() as u64, 43);

    let c2 = counter.clone();
    let cv2 = counter_vec.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(500));
            c2.inc();
            cv2.with_label_values(&["3", "4"]).inc();
        }
    });

    thread::spawn(move || {
        for _ in 0..5 {
            thread::sleep(Duration::from_secs(1));
            counter.inc();
            counter_vec.with_label_values(&["3", "4"]).inc();
        }
    });

    // Choose your writer and encoder.
    let mut buffer = Vec::<u8>::new();
    let encoder = TextEncoder::new();
    for _ in 0..5 {
        let metric_familys = r.gather();
        encoder.encode(&metric_familys, &mut buffer).unwrap();

        // Output to the standard output.
        println!("{}", String::from_utf8(buffer.clone()).unwrap());

        buffer.clear();
        thread::sleep(Duration::from_secs(1));
    }
}
