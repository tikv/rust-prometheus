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

/// Matches `... => { ... name: value_string_literal ... }`
#[derive(Debug)]
struct MetricValueDefFull {
    name: Ident,
    value: LitStr,
}

impl Synom for MetricValueDefFull {
    named!(parse -> Self, do_parse!(
        name: syn!(Ident) >>
        punct!(:) >>
        value: syn!(LitStr) >>
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

/// Matches either `... => { ... name: value_string_literal ... }` or `... => { ... value ... }`
#[derive(Debug)]
pub struct MetricValueDef {
    pub name: Ident,
    pub value: LitStr,
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
        MetricValueDef {
            name: e.value,
            value: LitStr::new(e.value.as_ref(), e.value.span()),
        }
    }
}

/// Matches `{ value_def, value_def, ... }`
#[derive(Debug)]
pub struct MetricValueDefList(Vec<MetricValueDef>);

impl Synom for MetricValueDefList {
    named!(parse -> Self, do_parse!(
        body: braces!(Punctuated::<MetricValueDef, Token![,]>::parse_terminated_nonempty) >>
        (MetricValueDefList(body.1.into_iter().collect()))
    ));
}

impl MetricValueDefList {
    pub fn get(&self) -> &Vec<MetricValueDef> {
        &self.0
    }

    pub fn get_names<'a>(&'a self) -> Vec<&'a Ident> {
        self.0.iter().map(|v| &v.name).collect()
    }

    pub fn get_values<'a>(&'a self) -> Vec<&'a LitStr> {
        self.0.iter().map(|v| &v.value).collect()
    }
}

/// Matches `(pub) label_enum Foo { value_def, value_def, ... }`
#[derive(Debug)]
pub struct MetricEnumDef {
    pub visibility: Visibility,
    pub enum_name: Ident,
    pub definitions: MetricValueDefList,
}

impl Synom for MetricEnumDef {
    named!(parse -> Self, do_parse!(
        visibility: syn!(Visibility) >>
        syn!(LabelEnum) >>
        enum_name: syn!(Ident) >>
        definitions: syn!(MetricValueDefList) >>
        (MetricEnumDef {
            visibility,
            enum_name,
            definitions,
        })
    ));
}

impl MetricEnumDef {
    /// Builds `enum_name::enum_item`.
    pub fn build_fields_with_path(&self) -> Vec<Path> {
        self.definitions
            .get()
            .iter()
            .map(|v| {
                let mut segments = Punctuated::new();
                segments.push(PathSegment {
                    ident: self.enum_name.clone(),
                    arguments: PathArguments::None,
                });
                segments.push(PathSegment {
                    ident: v.name.clone(),
                    arguments: PathArguments::None,
                });
                Path {
                    leading_colon: None,
                    segments,
                }
            })
            .collect()
    }
}

#[derive(Debug)]
enum MetricLabelArm {
    ValueDefinitionList(MetricValueDefList),
    EnumReference(Ident),
}

/// Matches `label_key => { value_def, value_def, ... }` or
///         `label_key => enum_name`
#[derive(Debug)]
pub struct MetricLabelDef {
    pub label_key: LitStr,
    arm: MetricLabelArm,
}

impl Synom for MetricLabelDef {
    named!(parse -> Self, do_parse!(
        label: syn!(LitStr) >>
        punct!(=>) >>
        arm: alt!(
            syn!(MetricValueDefList) => { |values| MetricLabelArm::ValueDefinitionList(values) }
            |
            syn!(Ident) => { |ident| MetricLabelArm::EnumReference(ident) }
        ) >>
        (MetricLabelDef {
            label_key: label,
            arm,
        })
    ));
}

impl MetricLabelDef {
    /// Get (or lookup if label is defined using enums) the value definition list.
    pub fn get_value_def_list<'a>(
        &'a self,
        enum_definitions: &'a HashMap<Ident, MetricEnumDef>,
    ) -> &'a MetricValueDefList {
        match &self.arm {
            MetricLabelArm::ValueDefinitionList(ref v) => v,
            MetricLabelArm::EnumReference(ref e) => &enum_definitions.get(e).unwrap().definitions,
        }
    }

    /// Get the enum identifier if label is defined using enums.
    pub fn get_enum_ident<'a>(&'a self) -> Option<&'a Ident> {
        match &self.arm {
            MetricLabelArm::ValueDefinitionList(_) => None,
            MetricLabelArm::EnumReference(ref e) => Some(e),
        }
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
