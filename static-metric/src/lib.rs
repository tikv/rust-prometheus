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

#![feature(proc_macro)]

#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
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
