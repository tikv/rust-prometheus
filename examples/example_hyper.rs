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
use std::time::{Duration, Instant};

use hyper::header::ContentType;
use hyper::server::{Server, Request, Response};
use hyper::mime::Mime;

use prom::encoder::{Encoder, TextEncoder};
use prom::{Counter, Opts, Registry, Histogram, HistogramOpts};

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
            thread::sleep(Duration::from_millis(300));
            c2.inc();
        }
    });

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1500));
            counter.inc();
        }
    });

    let encoder = TextEncoder::new();
    // Http server
    run("127.0.0.1:9898", r, encoder);
}

// run runs a http server with a Registry and a Encoder, it blocks current thread.
pub fn run(addr: &str, registry: Registry, encoder: TextEncoder) {
    let opts = HistogramOpts::new("text_encoder_encode_durations_histogram_millisecond",
                                  "test help");
    let histogram = Histogram::with_opts(opts).unwrap();
    registry.register(Box::new(histogram.clone())).unwrap();

    println!("listening addr {:?}", addr);
    Server::http(addr)
        .unwrap()
        .handle(move |_: Request, mut res: Response| {
            let start = Instant::now();

            let metric_familys = registry.gather();
            let mut buffer = vec![];
            encoder.encode(&metric_familys, &mut buffer).unwrap();

            let spend = (start.elapsed().subsec_nanos() as f64) / 1e6;
            histogram.observe(spend);

            res.headers_mut()
                .set(ContentType(encoder.format_type().parse::<Mime>().unwrap()));
            res.send(&buffer).unwrap();
        })
        .unwrap();
}
