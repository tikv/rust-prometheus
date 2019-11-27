// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/*!
This crate provides staticly built metrics to your Prometheus application.

This is useful since it reduces the amount of branching and processing needed at runtime to collect metrics.

```rust
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prometheus;
use prometheus::{self, IntCounter, TextEncoder, Encoder};

lazy_static! {
    static ref HIGH_FIVE_COUNTER: IntCounter =
        register_int_counter!("highfives", "Number of high fives recieved").unwrap();
}

HIGH_FIVE_COUNTER.inc();
assert_eq!(HIGH_FIVE_COUNTER.get(), 1);
```

Is it reccomended that you consult the [`prometheus` documentation for more information.](https://docs.rs/prometheus/)
*/

#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

mod builder;
mod parser;
mod register_macro;
mod util;

use proc_macro::TokenStream;

use self::builder::TokensBuilder;
use self::parser::StaticMetricMacroBody;
use self::register_macro::RegisterMethodInvoking;

/// Build static metrics.
#[proc_macro]
pub fn make_static_metric(input: TokenStream) -> TokenStream {
    let body: StaticMetricMacroBody = syn::parse(input).unwrap();
    TokensBuilder::build(body).into()
}

/// Register a `CounterVec` and create static metrics from it.
#[proc_macro]
pub fn register_static_counter_vec(input: TokenStream) -> TokenStream {
    register_static_vec("counter", input)
}

/// Register a `IntCounterVec` and create static metrics from it.
#[proc_macro]
pub fn register_static_int_counter_vec(input: TokenStream) -> TokenStream {
    register_static_vec("int_counter", input)
}

/// Register a `GaugeVec` and create static metrics from it.
#[proc_macro]
pub fn register_static_gauge_vec(input: TokenStream) -> TokenStream {
    register_static_vec("gauge", input)
}

/// Register a `IntGaugeVec` and create static metrics from it.
#[proc_macro]
pub fn register_static_int_gauge_vec(input: TokenStream) -> TokenStream {
    register_static_vec("int_gauge", input)
}

/// Register a `HistogramVec` and create static metrics from it.
#[proc_macro]
pub fn register_static_histogram_vec(input: TokenStream) -> TokenStream {
    register_static_vec("histogram", input)
}

/// Procedural macro handler for `register_static_xxx_vec!`.
fn register_static_vec(register_type: &str, input: TokenStream) -> TokenStream {
    let invoking: RegisterMethodInvoking = syn::parse(input).unwrap();
    invoking.into_tokens(register_type).into()
}
