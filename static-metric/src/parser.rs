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

use std::collections::HashMap;

use proc_macro2::Span;
use syn::buffer::Cursor;
use syn::punctuated::Punctuated;
use syn::synom::{PResult, Synom};
use syn::*;

/// Matches `label_enum` keyword.
struct LabelEnum {
    pub span: Span,
}

impl Synom for LabelEnum {
    fn parse(tokens: Cursor) -> PResult<Self> {
        if let Some((term, rest)) = tokens.term() {
            if term.as_str() == "label_enum" {
                return Ok((LabelEnum { span: term.span() }, rest));
            }
        }
        parse_error()
    }

    fn description() -> Option<&'static str> {
        Some("label_enum")
    }
}

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
        let value_lit = Lit::from(LitStr::new(e.value.as_ref(), e.value.span()));
        MetricValueDef {
            name: e.value,
            value: Expr::from(ExprLit {
                attrs: vec![],
                lit: value_lit,
            }),
        }
    }
}

/// Matches `{ value, value, ... }`
#[derive(Debug)]
struct MetricValueList {
    pub values: Vec<MetricValueDef>,
}

impl Synom for MetricValueList {
    named!(parse -> Self, do_parse!(
        body: braces!(Punctuated::<MetricValueDef, Token![,]>::parse_terminated_nonempty) >>
        (MetricValueList {
            values: body.1.into_iter().collect(),
        })
    ));
}

/// Matches `label_enum Foo { value_definition, value_definition, ... }`
#[derive(Debug)]
pub struct MetricEnumDef {
    pub enum_name: Ident,
    values: MetricValueList,
}

impl Synom for MetricEnumDef {
    named!(parse -> Self, do_parse!(
        syn!(LabelEnum) >>
        enum_name: syn!(Ident) >>
        values: syn!(MetricValueList) >>
        (MetricEnumDef {
            enum_name,
            values,
        })
    ));
}

impl MetricEnumDef {
    pub fn get_values(&self) -> &Vec<MetricValueDef> {
        &self.values.values
    }
}

#[derive(Debug)]
enum MetricLabelValuesOrEnum {
    Values(MetricValueList),
    Enum(Ident),
}

impl MetricLabelValuesOrEnum {
    fn get_values<'a>(
        &'a self,
        enum_definitions: &'a HashMap<Ident, MetricEnumDef>,
    ) -> &'a Vec<MetricValueDef> {
        match *self {
            MetricLabelValuesOrEnum::Values(ref v) => &v.values,
            MetricLabelValuesOrEnum::Enum(ref e) => {
                let enum_definition = enum_definitions.get(e);
                if enum_definition.is_none() {
                    panic!("label enum {} is undefined", e)
                }
                &enum_definition.unwrap().get_values()
            }
        }
    }
}

/// Matches `label_key => { value_definition, value_definition, ... }` or
///         `label_key => enum_name`
#[derive(Debug)]
pub struct MetricLabelDef {
    pub label_key: LitStr,
    values_or_enum: MetricLabelValuesOrEnum,
}

impl Synom for MetricLabelDef {
    named!(parse -> Self, do_parse!(
        label: syn!(LitStr) >>
        punct!(=>) >>
        values_or_enum: alt!(
            syn!(MetricValueList) => { |values| MetricLabelValuesOrEnum::Values(values) }
            |
            syn!(Ident) => { |ident| MetricLabelValuesOrEnum::Enum(ident) }
        ) >>
        (MetricLabelDef {
            label_key: label,
            values_or_enum,
        })
    ));
}

impl MetricLabelDef {
    pub fn get_values<'a>(
        &'a self,
        enum_definitions: &'a HashMap<Ident, MetricEnumDef>,
    ) -> &'a Vec<MetricValueDef> {
        self.values_or_enum.get_values(enum_definitions)
    }
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
pub enum StaticMetricMacroBodyItem {
    Metric(MetricDef),
    Enum(MetricEnumDef),
}

#[derive(Debug)]
pub struct StaticMetricMacroBody {
    pub items: Vec<StaticMetricMacroBodyItem>,
}

impl Synom for StaticMetricMacroBody {
    named!(parse -> Self, do_parse!(
        items: many0!(alt!(
            syn!(MetricDef) => { |m| StaticMetricMacroBodyItem::Metric(m) }
            |
            syn!(MetricEnumDef) => { |e| StaticMetricMacroBodyItem::Enum(e) }
        )) >>
        (StaticMetricMacroBody { items })
    ));
}
