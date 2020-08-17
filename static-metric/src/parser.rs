// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::collections::HashMap;

use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::*;
use syn::*;

/// Matches `label_enum` keyword.
struct LabelEnum {
    pub span: Span,
}

impl Parse for LabelEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        if &ident == "label_enum" {
            return Ok(LabelEnum { span: ident.span() });
        }
        Err(input.error("Expected `label_enum`"))
    }
}

/// Matches `... => { ... name: value_string_literal ... }`
#[derive(Debug)]
struct MetricValueDefFull {
    name: Ident,
    value: LitStr,
}

impl Parse for MetricValueDefFull {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let _: Colon = input.parse()?;
        let value = input.parse()?;
        Ok(MetricValueDefFull { name, value })
    }
}

/// Matches `... => { ... value ... }`
#[derive(Debug)]
struct MetricValueDefShort {
    value: Ident,
}

impl Parse for MetricValueDefShort {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MetricValueDefShort {
            value: input.parse()?,
        })
    }
}

/// Matches either `... => { ... name: value_string_literal ... }` or `... => { ... value ... }`
#[derive(Debug)]
pub struct MetricValueDef {
    pub name: Ident,
    pub value: LitStr,
}

impl Parse for MetricValueDef {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek2(Colon) {
            let full: MetricValueDefFull = input.parse()?;
            Ok(full.into())
        } else {
            let short: MetricValueDefShort = input.parse()?;
            Ok(short.into())
        }
    }
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
            value: LitStr::new(&e.value.to_string(), e.value.span()),
            name: e.value,
        }
    }
}

/// Matches `{ value_def, value_def, ... }`
#[derive(Debug)]
pub struct MetricValueDefList(Vec<MetricValueDef>);

impl Parse for MetricValueDefList {
    fn parse(input: ParseStream) -> Result<Self> {
        let body;
        let _ = braced!(body in input);
        let p = Punctuated::<MetricValueDef, Token![,]>::parse_terminated(&body)?;
        Ok(MetricValueDefList(p.into_iter().collect()))
    }
}

impl MetricValueDefList {
    pub fn get(&self) -> &Vec<MetricValueDef> {
        &self.0
    }

    pub fn get_names(&self) -> Vec<&Ident> {
        self.0.iter().map(|v| &v.name).collect()
    }

    pub fn get_values(&self) -> Vec<&LitStr> {
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

impl Parse for MetricEnumDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility = input.parse()?;
        let _: LabelEnum = input.parse()?;
        let enum_name = input.parse()?;
        let definitions = input.parse()?;
        Ok(MetricEnumDef {
            visibility,
            enum_name,
            definitions,
        })
    }
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

impl Parse for MetricLabelDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let label_key = input.parse()?;
        let _: FatArrow = input.parse()?;
        let arm = if input.peek(Brace) {
            MetricLabelArm::ValueDefinitionList(input.parse()?)
        } else {
            MetricLabelArm::EnumReference(input.parse()?)
        };
        Ok(MetricLabelDef { label_key, arm })
    }
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
    pub fn get_enum_ident(&self) -> Option<&Ident> {
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

impl Parse for MetricDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility = input.parse()?;
        let _: Struct = input.parse()?;
        let struct_name = input.parse()?;
        let _: Colon = input.parse()?;
        let metric_type = input.parse()?;

        let body;
        let _ = braced!(body in input);
        let p = Punctuated::<MetricLabelDef, Token![,]>::parse_terminated(&body)?;
        let labels = p.into_iter().collect();

        Ok(MetricDef {
            visibility,
            struct_name,
            metric_type,
            labels,
        })
    }
}

#[derive(Debug)]
pub enum StaticMetricMacroBodyItem {
    Metric(MetricDef),
    Enum(MetricEnumDef),
}

impl Parse for StaticMetricMacroBodyItem {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek2(Token!(struct)) || input.peek2(Token!(struct)) {
            Ok(StaticMetricMacroBodyItem::Metric(input.parse()?))
        } else {
            Ok(StaticMetricMacroBodyItem::Enum(input.parse()?))
        }
    }
}

#[derive(Debug)]
pub struct StaticMetricMacroBody {
    pub items: Vec<StaticMetricMacroBodyItem>,
}

impl Parse for StaticMetricMacroBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(StaticMetricMacroBody { items })
    }
}
