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

Please take a look at [examples](./examples) directory for more.
