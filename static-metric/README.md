# prometheus-static-metric

[![docs.rs](https://docs.rs/prometheus-static-metric/badge.svg)](https://docs.rs/prometheus-static-metric)
[![crates.io](http://meritbadge.herokuapp.com/prometheus-static-metric)](https://crates.io/crates/prometheus-static-metric)

Utility macro to build static metrics for the [rust-prometheus](https://github.com/pingcap/rust-prometheus) library.

## Why?

`MetricVec` (i.e. `CounterVec`, `GaugeVec` or `HistogramVec`) is slow. However if every possible values for labels are
known, each metric in the `MetricVec` can be cached to avoid the runtime cost.

For example, the following code can be slow when it is invoked multiple times:

```rust
some_counter_vec.with_label_values(&["label_1_foo", "label_2_bar"]).inc();
```

It is because we are retriving a specific `Counter` according to values each time and to ensure thread-safety there is
a lock inside which makes things worse.

We can optimize it by caching the counter by label values:

```rust
// init before hand
let foo_bar_counter = some_counter.with_label_values(&["label_1_foo", "label_2_bar"]);

foo_bar_counter.inc();
```

So far everything seems good. We achieve the same performance as `Counter` for `CounterVec`. But what if there are many
labels and each of them has many values? We need to hand-craft a lot of code in this way.

That's what this crate solves. This crate provides a macro that helps you do the optimization above without really
introducing a lot of templating code.

## Getting Started

+ Add to `Cargo.toml`:

    ```toml
    [dependencies]
    prometheus-static-metric = "0.1"
    ```

+ Add to `lib.rs`:

    ```rust
    #![feature(proc_macro)]

    extern crate prometheus_static_metric;
    ```

## Example

Use the `make_static_metric!` to define all possible values for each label. Your definition will be expanded to a real
`struct` for easy access while keeping high-performance.

```rust
use prometheus_static_metric::make_static_metric;

make_static_metric! {
    pub struct MyStaticCounterVec: Counter {
        "method" => {
            post,
            get,
            put,
            delete,
        },
        "product" => {
            foo,
            bar,
        },
    }
}

fn main() {
    let vec = CounterVec::new(Opts::new("foo", "bar"), &["method", "product"]).unwrap();
    let static_counter_vec = MyStaticCounterVec::from(&vec);

    static_counter_vec.post.foo.inc();
    static_counter_vec.delete.bar.inc_by(4.0);
    assert_eq!(static_counter_vec.post.bar.get(), 0.0);
    assert_eq!(vec.with_label_values(&["post", "foo"]).get(), 1.0);
    assert_eq!(vec.with_label_values(&["delete", "bar"]).get(), 4.0);
}
```

## Auto-flush-able local threaded mertric

For heavier scenario that a global shared static-metric might not be effecient enough, you can use `make_auto_flush_static_metric!` macro, which will store data in local thread storage, with a custom rate to flush to global `MetricVec`.

```rust
#[macro_use]
extern crate lazy_static;
extern crate prometheus;
extern crate prometheus_static_metric;

use prometheus::*;
use prometheus_static_metric::auto_flush_from;
use prometheus_static_metric::make_auto_flush_static_metric;

make_auto_flush_static_metric! {

    pub label_enum FooBar {
        foo,
        bar,
    }

    pub label_enum Methods {
        post,
        get,
        put,
        delete,
    }

    pub struct Lhrs: LocalIntCounter {
        "product" => FooBar,
        "method" => Methods,
        "version" => {
            http1: "HTTP/1",
            http2: "HTTP/2",
        },
    }
}

lazy_static! {
    pub static ref HTTP_COUNTER_VEC: IntCounterVec =
        register_int_counter_vec ! (
            "http_requests",
            "Total number of HTTP requests.",
            & ["product", "method", "version"]    // it doesn't matter for the label order
        ).unwrap();
}

lazy_static! {
    // You can also use default flush duration which is 1 second.
    // pub static ref TLS_HTTP_COUNTER: Lhrs = auto_flush_from!(HTTP_COUNTER_VEC, Lhrs);
    pub static ref TLS_HTTP_COUNTER: Lhrs = auto_flush_from!(HTTP_COUNTER_VEC, Lhrs, std::time::Duration::from_secs(1));
}

fn main() {
    TLS_HTTP_COUNTER.foo.post.http1.inc();
    TLS_HTTP_COUNTER.foo.post.http1.inc();

    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        0
    );

    ::std::thread::sleep(::std::time::Duration::from_secs(2));

    TLS_HTTP_COUNTER.foo.post.http1.inc();
    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        3
    );
}
```



Please take a look at [examples](./examples) directory for more.