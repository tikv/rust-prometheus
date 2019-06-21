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
#![feature(duration_float)]

extern crate test;

use std::collections::HashMap;
use std::sync::{atomic, Arc};
use std::thread;
use std::cell::Cell;

use prometheus::*;
use prometheus::local::*;
use prometheus::histogram::Instant as PromInstant;
use test::Bencher;

const CONCURRENT_BENCH_CONCURRENCY: usize = 4;

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

#[bench]
fn bench_counter_no_labels_concurrent_nop(b: &mut Bencher) {
    let signal_exit = Arc::new(atomic::AtomicBool::new(false));
    let counter = Counter::new("foo", "bar").unwrap();

    let thread_handles: Vec<_> = (0..CONCURRENT_BENCH_CONCURRENCY)
        .map(|_| {
            let signal_exit2 = signal_exit.clone();
            thread::spawn(move || {
                while !signal_exit2.load(atomic::Ordering::Relaxed) {
                    // Do nothing as the control group.
                }
            })
        })
        .collect();

    b.iter(|| counter.inc());

    // Wait for accompanying thread to exit.
    signal_exit.store(true, atomic::Ordering::Relaxed);
    for h in thread_handles {
        h.join().unwrap();
    }
}

#[bench]
fn bench_counter_no_labels_concurrent_write(b: &mut Bencher) {
    let signal_exit = Arc::new(atomic::AtomicBool::new(false));
    let counter = Counter::new("foo", "bar").unwrap();

    let thread_handles: Vec<_> = (0..CONCURRENT_BENCH_CONCURRENCY)
        .map(|_| {
            let signal_exit2 = signal_exit.clone();
            let counter2 = counter.clone();
            thread::spawn(move || {
                while !signal_exit2.load(atomic::Ordering::Relaxed) {
                    // Update counter concurrently as the normal group.
                    counter2.inc();
                }
            })
        })
        .collect();

    b.iter(|| counter.inc());

    // Wait for accompanying thread to exit.
    signal_exit.store(true, atomic::Ordering::Relaxed);
    for h in thread_handles {
        h.join().unwrap();
    }
}

#[bench]
fn bench_int_counter_no_labels_concurrent_write(b: &mut Bencher) {
    let signal_exit = Arc::new(atomic::AtomicBool::new(false));
    let counter = IntCounter::new("foo", "bar").unwrap();

    let thread_handles: Vec<_> = (0..CONCURRENT_BENCH_CONCURRENCY)
        .map(|_| {
            let signal_exit2 = signal_exit.clone();
            let counter2 = counter.clone();
            thread::spawn(move || {
                while !signal_exit2.load(atomic::Ordering::Relaxed) {
                    // Update counter concurrently as the normal group.
                    counter2.inc();
                }
            })
        })
        .collect();

    b.iter(|| counter.inc());

    // Wait for accompanying thread to exit.
    signal_exit.store(true, atomic::Ordering::Relaxed);
    for h in thread_handles {
        h.join().unwrap();
    }
}

#[bench]
fn bench_counter_with_label_values_concurrent_write(b: &mut Bencher) {
    let signal_exit = Arc::new(atomic::AtomicBool::new(false));
    let counter = CounterVec::new(Opts::new("foo", "bar"), &["one", "two", "three"]).unwrap();

    let thread_handles: Vec<_> = (0..CONCURRENT_BENCH_CONCURRENCY)
        .map(|_| {
            let signal_exit2 = signal_exit.clone();
            let counter2 = counter.clone();
            thread::spawn(move || {
                while !signal_exit2.load(atomic::Ordering::Relaxed) {
                    counter2.with_label_values(&["eins", "zwei", "drei"]).inc();
                }
            })
        })
        .collect();

    b.iter(|| counter.with_label_values(&["eins", "zwei", "drei"]).inc());

    // Wait for accompanying thread to exit.
    signal_exit.store(true, atomic::Ordering::Relaxed);
    for h in thread_handles {
        h.join().unwrap();
    }
}

lazy_static::lazy_static! {
    pub static ref HTTP_COUNTER: IntCounter = register_int_counter!("foo", "Bar").unwrap();
}

thread_local! {
    static THREAD_LAST_TICK_TIME: Cell<PromInstant> = Cell::new(PromInstant::now_coarse());

    static COARSETIME_THREAD_LAST_TICK_TIME: Cell<coarsetime::Instant> = Cell::new(coarsetime::Instant::now());

    pub static TLS_HTTP_COUNTER: LocalIntCounter = HTTP_COUNTER.local();
}

fn flush_1() {
    THREAD_LAST_TICK_TIME.with(|tls_last_tick| {
        let last_tick = tls_last_tick.get();
        if last_tick.elapsed().as_secs_f64() < 0.1 {
            return;
        }
        tls_last_tick.set(PromInstant::now_coarse());
        TLS_HTTP_COUNTER.with(|m| m.flush());
    });
}

fn flush_2() {
    COARSETIME_THREAD_LAST_TICK_TIME.with(|tls_last_tick| {
        let now = coarsetime::Instant::recent();
        let last_tick = tls_last_tick.get();
        if now.duration_since(last_tick).as_f64() < 0.1 {
            return;
        }
        tls_last_tick.set(now);
        TLS_HTTP_COUNTER.with(|m| m.flush());
    });
}

#[bench]
fn bench_local_counter_flush_concurrent_1(b: &mut Bencher) {
    let signal_exit = Arc::new(atomic::AtomicBool::new(false));

    let thread_handles: Vec<_> = (0..CONCURRENT_BENCH_CONCURRENCY)
        .map(|_| {
            let signal_exit2 = signal_exit.clone();
            thread::spawn(move || {
                while !signal_exit2.load(atomic::Ordering::Relaxed) {
                    TLS_HTTP_COUNTER.with(|m| {
                        m.inc();
                    });
                    flush_1();
                }
            })
        })
        .collect();

    b.iter(|| {
        TLS_HTTP_COUNTER.with(|m| {
            m.inc();
        });
        flush_1();
    });

    // Wait for accompanying thread to exit.
    signal_exit.store(true, atomic::Ordering::Relaxed);
    for h in thread_handles {
        h.join().unwrap();
    }
}

#[bench]
fn bench_local_counter_flush_concurrent_2(b: &mut Bencher) {
    let signal_exit = Arc::new(atomic::AtomicBool::new(false));

    let updater = coarsetime::Updater::new(5).start().unwrap();

    let thread_handles: Vec<_> = (0..CONCURRENT_BENCH_CONCURRENCY)
        .map(|_| {
            let signal_exit2 = signal_exit.clone();
            thread::spawn(move || {
                while !signal_exit2.load(atomic::Ordering::Relaxed) {
                    TLS_HTTP_COUNTER.with(|m| {
                        m.inc();
                    });
                    flush_2();
                }
            })
        })
        .collect();

    b.iter(|| {
        TLS_HTTP_COUNTER.with(|m| {
            m.inc();
        });
        flush_2();
    });

    // Wait for accompanying thread to exit.
    signal_exit.store(true, atomic::Ordering::Relaxed);
    for h in thread_handles {
        h.join().unwrap();
    }

    updater.stop().unwrap();
}

