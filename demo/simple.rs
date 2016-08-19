// Copyright 2014 The Prometheus Authors
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

extern crate prom;
extern crate hyper;

use std::thread;
use std::time::Duration;

use prom::*;

fn main() {
    let opts = Opts::new("test", "test help").const_label("a", "1").const_label("b", "2");
    let counter = Counter::with_opts(opts).unwrap();

    let r = Registry::new();
    r.register(Box::new(counter.clone())).unwrap();

    counter.inc();
    assert_eq!(counter.get() as u64, 1);
    counter.inc_by(42.0).unwrap();
    assert_eq!(counter.get() as u64, 43);

    let c2 = counter.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(400));
            c2.inc_by(0.33333333e-8).unwrap();
        }
    });

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(300));
            counter.inc_by(0.1111111111e-4).unwrap();
        }
    });

    // Http server
    prom::http::run_with_registry("127.0.0.1:9898", &r).unwrap();
}
