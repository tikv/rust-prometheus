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

use syn::punctuated::Punctuated;
use syn::synom::Synom;
use syn::{Expr, ExprLit, Ident, Lit, LitStr, Visibility};

/// Matches `... => { ... name: value_expr ... }`
#[derive(Debug)]
struct MetricValueDefFull {
    name: Ident,
    value: Expr,
}

impl Synom for MetricValueDefFull {
    named!(parse -> Self, do_parse!(
        name: syn!(Ident) >>
        punct!(:) >>
        value: syn!(Expr) >>
        (MetricValueDefFull { name, value })
    ));
}

/// Matches `... => { ... value ... }`
#[derive(Debug)]
struct MetricValueDefShort {
    value: Ident,
}

impl Synom for MetricValueDefShort {
    named!(parse -> Self, do_parse!(
        value: syn!(Ident) >>
        (MetricValueDefShort { value })
    ));
}

/// Matches either `... => { ... name: value_expr ... }` or `... => { ... value ... }`
#[derive(Debug)]
pub struct MetricValueDef {
    pub name: Ident,
    pub value: Expr,
}

impl Synom for MetricValueDef {
    named!(parse -> Self, alt!(
        syn!(MetricValueDefFull) => { MetricValueDef::from }
        |
        syn!(MetricValueDefShort) => { MetricValueDef::from }
    ));
}

impl From<MetricValueDefFull> for MetricValueDef {
    fn from(e: MetricValueDefFull) -> MetricValueDef {
        MetricValueDef {
            name: e.name,
            value: e.value,
        }
    }
}

impl From<MetricValueDefShort> for MetricValueDef {
    fn from(e: MetricValueDefShort) -> MetricValueDef {
        let value_lit = Lit::from(LitStr::new(e.value.as_ref(), e.value.span));
        MetricValueDef {
            name: e.value,
            value: Expr::from(ExprLit {
                attrs: vec![],
                lit: value_lit,
            }),
        }
    }
}

/// Matches `label_key => { value_definition, value_definition, ... }`
#[derive(Debug)]
pub struct MetricLabelDef {
    pub label_key: LitStr,
    pub values: Vec<MetricValueDef>,
}

impl Synom for MetricLabelDef {
    named!(parse -> Self, do_parse!(
        label: syn!(LitStr) >>
        punct!(=>) >>
        body: braces!(Punctuated::<MetricValueDef, Token![,]>::parse_terminated_nonempty) >>
        (MetricLabelDef {
            label_key: label,
            values: body.1.into_iter().collect(),
        })
    ));
}

/// Matches `(pub) struct Foo: Counter { a => { ... }, b => { ... }, ... }`
#[derive(Debug)]
pub struct MetricDef {
    pub visibility: Visibility,
    pub struct_name: Ident,
    pub metric_type: Ident,
    pub labels: Vec<MetricLabelDef>,
}

impl Synom for MetricDef {
    named!(parse -> Self, do_parse!(
        visibility: syn!(Visibility) >>
        keyword!(struct) >>
        struct_name: syn!(Ident) >>
        punct!(:) >>
        metric_type: syn!(Ident) >>
        body: braces!(Punctuated::<MetricLabelDef, Token![,]>::parse_terminated_nonempty) >>
        (MetricDef {
            visibility,
            struct_name,
            metric_type,
            labels: body.1.into_iter().collect(),
        })
    ));
}

#[derive(Debug)]
pub struct StaticMetricMacroBody {
    pub metrics: Vec<MetricDef>,
}

impl Synom for StaticMetricMacroBody {
    named!(parse -> Self, do_parse!(
        metrics: many0!(syn!(MetricDef)) >>
        (StaticMetricMacroBody { metrics })
    ));
}
