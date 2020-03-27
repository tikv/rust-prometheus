// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::collections::HashMap;
use std::sync::atomic::Ordering;

use proc_macro2::{Span, TokenStream as Tokens};
use quote::{ToTokens, TokenStreamExt};
use syn::{Ident, Visibility};

use super::parser::*;
use super::util;
use crate::builder::TokensBuilder;
use crate::builder::SCOPE_ID;

pub struct AutoFlushTokensBuilder;

impl AutoFlushTokensBuilder {
    pub fn build(macro_body: StaticMetricMacroBody) -> Tokens {
        let mut enums_definitions = HashMap::new();
        let mut tokens = Tokens::new();
        for item in macro_body.items {
            match item {
                StaticMetricMacroBodyItem::Metric(m) => {
                    // If this is a metric definition, expand to a `struct`.
                    tokens.append_all(Self::build_metric_struct(&m, &enums_definitions));
                }
                StaticMetricMacroBodyItem::Enum(e) => {
                    // If this is a label enum definition, expand to an `enum` and
                    // add to the collection.
                    tokens.append_all(TokensBuilder::build_label_enum(&e));
                    enums_definitions.insert(e.enum_name.clone(), e);
                }
            }
        }
        tokens
    }

    fn build_metric_struct(
        metric: &MetricDef,
        enum_definitions: &HashMap<Ident, MetricEnumDef>,
    ) -> Tokens {
        // Check `label_enum` references.
        for label in &metric.labels {
            let enum_ident = label.get_enum_ident();
            if let Some(e) = enum_ident {
                // If metric is using a `label_enum`, it must exist before the metric definition.
                let enum_def = enum_definitions.get(e);
                if enum_def.is_none() {
                    panic!("Label enum `{}` is undefined.", e)
                }

                // If metric has `pub` visibility, then `label_enum` should also be `pub`.
                // TODO: Support other visibility, like `pub(xx)`.
                if let Visibility::Public(_) = metric.visibility {
                    if let Visibility::Public(_) = enum_def.unwrap().visibility {
                        // `pub` is ok.
                    } else {
                        // others are unexpected.
                        panic!(
                            "Label enum `{}` does not have enough visibility because it is \
                             used in metric `{}` which has `pub` visibility.",
                            e, metric.struct_name
                        );
                    }
                }
            }
        }

        let label_struct: Vec<_> = metric
            .labels
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let builder_context = MetricBuilderContext::new(metric, enum_definitions, i);
                let inner_struct = builder_context.build_inner_struct();
                let inner_impl = builder_context.build_inner_impl();
                let inner_trait_impl = builder_context.build_inner_trait_impl();
                let delegator_struct = builder_context.build_delegator_struct();
                let delegator_impl = builder_context.build_delegator_impl();
                quote! {
                    #inner_struct
                    #inner_impl
                    #inner_trait_impl
                    #delegator_struct
                    #delegator_impl
                }
            })
            .collect();

        let builder_contexts: Vec<MetricBuilderContext> = metric
            .labels
            .iter()
            .enumerate()
            .map(|(i, _)| MetricBuilderContext::new(metric, enum_definitions, i))
            .collect();

        let auto_flush_delegator: Tokens =
            Self::build_auto_flush_delegator(metric, &builder_contexts);
        let outer_struct: Tokens = Self::build_outer_struct(&builder_contexts);
        let outer_impl: Tokens = Self::build_outer_impl(&builder_contexts);
        let scope_id = SCOPE_ID.fetch_add(1, Ordering::Relaxed);
        let scope_name = Ident::new(
            &format!("prometheus_static_scope_{}", scope_id),
            Span::call_site(),
        );

        let visibility = &metric.visibility;
        let inner_struct_name =
            Ident::new(&format!("{}Inner", &metric.struct_name), Span::call_site());
        let outer_struct_name = metric.struct_name.clone();

        quote! {
            #visibility use self::#scope_name::#inner_struct_name;
            #visibility use self::#scope_name::#outer_struct_name;

            #[allow(dead_code)]
            mod #scope_name {
                use ::std::collections::HashMap;
                use ::prometheus::*;
                use ::prometheus::local::*;
                use ::std::cell::Cell;
                use ::std::thread::LocalKey;
                use std::mem;
                use std::mem::MaybeUninit;


                #[allow(unused_imports)]
                use super::*;

                #(
                    #label_struct
                )*

                #auto_flush_delegator
                #outer_struct
                #outer_impl
            }
        }
    }

    fn build_auto_flush_delegator(
        metric: &MetricDef,
        builder_contexts: &[MetricBuilderContext],
    ) -> Tokens {
        let inner_struct = &builder_contexts[0].inner_struct_name();
        let last_builder_contexts = &builder_contexts
            .last()
            .expect("builder contexts should not be empty");
        let last_delegator = last_builder_contexts.delegator_struct_name();
        let metric_type = metric.metric_type.clone();

        fn offset_fetcher(builder_context: &MetricBuilderContext) -> Tokens {
            let struct_type = builder_context.inner_struct_name();
            let struct_var_name = Ident::new(
                struct_type.to_string().to_lowercase().as_str(),
                Span::call_site(),
            );

            let member_type = builder_context.inner_member_type.clone();
            let member_var_name = Ident::new(
                member_type.to_string().to_lowercase().as_str(),
                Span::call_site(),
            );
            let offset = Ident::new(
                &format!("offset{}", builder_context.label_index + 1),
                Span::call_site(),
            );
            let head = if builder_context.label_index == 0 {
                quote! {
                    let #struct_var_name = root_metric as *const #struct_type;
                }
            } else {
                Tokens::new()
            };

            let body = quote! {
                let #member_var_name = (#struct_var_name as usize + self.#offset) as *const #member_type;
            };

            let tail = if builder_context.is_last_label {
                quote! {
                    &*#member_var_name
                }
            } else {
                Tokens::new()
            };

            quote! {
                #head
                #body
                #tail
            }
        }

        let offset_fetchers = builder_contexts
            .iter()
            .map(|m| offset_fetcher(m))
            .collect::<Vec<Tokens>>();

        let delegator_tokens = if metric_type.to_string().contains("Counter") {
            quote! {
                CounterDelegator<#inner_struct, #metric_type>
            }
        } else if metric_type.to_string().contains("Histogram") {
            quote! {
                HistogramDelegator<#inner_struct>
            }
        } else {
            panic!("Metric type must be one of LocalCounter or LocalHistogram")
        };

        quote! {
            impl #delegator_tokens for #last_delegator {
                fn get_root_metric(&self) -> &'static LocalKey<#inner_struct> {
                    self.root
                }
                fn get_local<'a>(&self, root_metric: &'a #inner_struct) -> &'a #metric_type {
                   unsafe {
                       #(
                            #offset_fetchers
                       )*
                   }
                }
            }
        }
    }

    fn build_outer_struct(builder_contexts: &[MetricBuilderContext]) -> Tokens {
        builder_contexts[0].build_outer_struct()
    }

    fn build_outer_impl(builder_contexts: &[MetricBuilderContext]) -> Tokens {
        builder_contexts[0].build_outer_impl()
    }
}

struct MetricBuilderContext<'a> {
    metric: &'a MetricDef,
    enum_definitions: &'a HashMap<Ident, MetricEnumDef>,
    label: &'a MetricLabelDef,
    next_label: Option<&'a MetricLabelDef>,
    label_index: usize,
    is_last_label: bool,
    is_secondary_last_label: bool,
    root_struct_name: Ident,
    struct_name: Ident,
    delegator_member_type: Ident,
    inner_member_type: Ident,
    inner_next_member_type: Ident,
}

impl<'a> MetricBuilderContext<'a> {
    fn new(
        metric: &'a MetricDef,
        enum_definitions: &'a HashMap<Ident, MetricEnumDef>,
        label_index: usize,
    ) -> MetricBuilderContext<'a> {
        let is_last_label = label_index == metric.labels.len() - 1;
        let is_secondary_last_label =
            metric.labels.len() >= 2 && label_index == metric.labels.len() - 2;

        MetricBuilderContext {
            metric,
            enum_definitions,
            label: &metric.labels[label_index],
            next_label: metric.labels.get(label_index + 1),
            label_index,
            is_last_label,
            is_secondary_last_label,
            root_struct_name: util::get_label_struct_name(metric.struct_name.clone(), 0),
            struct_name: util::get_label_struct_name(metric.struct_name.clone(), label_index),
            delegator_member_type: util::get_delegator_member_type(
                metric.struct_name.clone(),
                label_index,
                is_last_label,
            ),
            inner_member_type: util::get_inner_member_type(
                metric.struct_name.clone(),
                label_index,
                metric.metric_type.clone(),
                is_last_label,
            ),
            inner_next_member_type: util::get_inner_member_type(
                metric.struct_name.clone(),
                label_index + 1,
                metric.metric_type.clone(),
                is_secondary_last_label,
            ),
        }
    }

    fn inner_struct_name(&self) -> Ident {
        Ident::new(&format!("{}Inner", &self.struct_name), Span::call_site())
    }

    fn delegator_struct_name(&self) -> Ident {
        Ident::new(
            &format!("{}Delegator", &self.struct_name),
            Span::call_site(),
        )
    }

    fn build_inner_struct(&self) -> Tokens {
        let struct_name = self.inner_struct_name();

        let field_names = self
            .label
            .get_value_def_list(self.enum_definitions)
            .get_names();
        let member_types: Vec<_> = field_names
            .iter()
            .map(|_| &self.inner_member_type)
            .collect();
        let last_flush = if self.label_index == 0 {
            quote! {
                last_flush: Cell<u64>,
                flush_millis: u64,
            }
        } else {
            Tokens::new()
        };

        quote! {
            #[allow(missing_copy_implementations)]
            pub struct #struct_name {
                #(
                    pub #field_names: #member_types,
                )*
                #last_flush
            }
        }
    }

    fn build_outer_struct(&self) -> Tokens {
        let metric_type = self.metric.metric_type.clone();
        let outer_struct_name = self.struct_name.clone();
        let inner_struct_name = self.inner_struct_name();
        let delegator_name = self.delegator_struct_name();
        let auto_flush_local = if metric_type.to_string().contains("Counter") {
            quote! {
                AFLocalCounter<#inner_struct_name,#metric_type,#delegator_name>
            }
        } else {
            quote! {
                AFLocalHistogram<#inner_struct_name,#delegator_name>
            }
        };

        let member_type = if self.metric.labels.len() == 1 {
            auto_flush_local
        } else {
            delegator_name.into_token_stream()
        };
        let field_names = self
            .label
            .get_value_def_list(self.enum_definitions)
            .get_names();

        quote! {
            pub struct #outer_struct_name {
                inner: &'static LocalKey<#inner_struct_name>,
                #(
                  pub #field_names: #member_type,
                )*
            }
        }
    }
    fn build_inner_impl(&self) -> Tokens {
        let struct_name = self.inner_struct_name();
        let impl_from = self.build_inner_impl_from();
        let impl_flush = self.build_inner_impl_flush();
        let update_flush_duration = self.build_with_flush_duration();

        quote! {
            impl #struct_name {
                #impl_from
                #impl_flush
                #update_flush_duration
            }
        }
    }

    fn build_delegator_impl(&self) -> Tokens {
        let struct_name = self.delegator_struct_name();
        let impl_new = self.build_delegator_impl_new();
        let impl_get = self.build_delegator_impl_get();

        quote! {
            impl #struct_name {
                #impl_new
                #impl_get
            }
        }
    }

    fn build_outer_impl(&self) -> Tokens {
        let outer_struct_name = self.struct_name.clone();

        let impl_from = self.build_outer_impl_from();
        let impl_get = self.build_outer_impl_get();
        quote! {
            impl #outer_struct_name {
                #impl_from
                #impl_get

                pub fn flush(&self) {
                    self.inner.with(|m| m.flush())
                }
            }
        }
    }

    fn build_inner_trait_impl(&self) -> Tokens {
        let struct_name = self.inner_struct_name();
        if self.label_index == 0 {
            quote! {
                impl ::prometheus::local::LocalMetric for #struct_name {
                    fn flush(&self) {
                        #struct_name::flush(self);
                    }
                }

                impl ::prometheus::local::MayFlush for #struct_name {
                    fn may_flush(&self) {
                        MayFlush::try_flush(self, &self.last_flush, self.flush_millis)
                    }
                }
            }
        } else {
            Tokens::new()
        }
    }

    fn build_inner_impl_from(&self) -> Tokens {
        let struct_name = self.inner_struct_name();
        let metric_vec_type = util::to_non_local_metric_type(util::get_metric_vec_type(
            self.metric.metric_type.clone(),
        ));

        let prev_labels_ident: Vec<_> = (0..self.label_index)
            .map(|i| Ident::new(&format!("label_{}", i), Span::call_site()))
            .collect();
        let body = self.build_inner_impl_from_body(&prev_labels_ident);

        quote! {
            pub fn from(
                #(
                    #prev_labels_ident: &str,
                )*
                m: &#metric_vec_type
            ) -> #struct_name {
                #struct_name {
                    #body
                }
            }
        }
    }

    fn build_delegator_impl_new(&self) -> Tokens {
        let inner_name = Ident::new(
            &format!("{}Inner", self.metric.struct_name),
            Span::call_site(),
        );
        let delegator_name = self.delegator_struct_name();
        let delegator_member = self.delegator_member_type.clone();
        let member_type = self.inner_member_type.clone();
        let next_member_type = self.inner_next_member_type.clone();
        let metric_type = self.metric.metric_type.clone();
        let known_offsets = (1..=(self.label_index + 1))
            .map(|m| Ident::new(&format!("offset{}", m), Span::call_site()))
            .collect::<Vec<Ident>>();
        let known_offsets_tokens = quote! {
          #(
          #known_offsets,
          )*
        };
        if self.is_last_label {
            let local_id = if metric_type.to_string().contains("Counter") {
                quote! {
                    AFLocalCounter
                }
            } else {
                quote! {
                    AFLocalHistogram
                }
            };

            let local_tokens = if metric_type.to_string().contains("Counter") {
                quote! {
                    #local_id<#inner_name,#metric_type,#delegator_name>
                }
            } else {
                quote! {
                    #local_id<#inner_name, #delegator_name>
                }
            };

            quote! {
            pub fn new(
                        root: &'static LocalKey<#inner_name>,
                            #(
                              #known_offsets : usize,
                            )*
                        ) -> #local_tokens {
                            let delegator = #delegator_name {
                                root,
                                #known_offsets_tokens
                            };
                           #local_id::new(delegator)
                        }
            }
        } else {
            let delegator_field_names = &self.delegator_field_names();

            quote! {
                pub fn new(
                    root: &'static LocalKey<#inner_name>,
                    #(
                      #known_offsets : usize,
                    )*
                ) -> #delegator_name {
                    let x = unsafe { MaybeUninit::<#member_type>::uninit().assume_init() };
                    let branch_offset = (&x as *const #member_type) as usize;
                    #(
                      let #delegator_field_names = #delegator_member::new(
                      root,
                      #known_offsets_tokens
                      &(x.#delegator_field_names) as *const #next_member_type as usize - branch_offset,
                      );
                    )*
                    mem::forget(x);
                    #delegator_name {
                        #(
                         #delegator_field_names,
                        )*
                    }
                }
            }
        }
    }

    fn build_outer_impl_from(&self) -> Tokens {
        let outer_struct_name = self.struct_name.clone();
        let inner_struct_name = self.inner_struct_name();
        let delegator_name = self.delegator_struct_name();
        let inner_member_type = self.inner_member_type.clone();
        let field_names = self
            .label
            .get_value_def_list(self.enum_definitions)
            .get_names();

        quote! {
            pub fn from(inner: &'static LocalKey<#inner_struct_name>) -> #outer_struct_name {
                let x = unsafe { MaybeUninit::<#inner_struct_name>::uninit().assume_init() };
                let branch_offset = &x as *const #inner_struct_name as usize;

                #(
                  let #field_names = #delegator_name::new(
                  &inner,
                  &(x.#field_names) as *const #inner_member_type as usize - branch_offset,
                  );
                )*
                mem::forget(x);

            #outer_struct_name {
             inner,
             #(
                #field_names,
             )*
            }
           }
        }
    }

    /// `fn get()` is only available when label is defined by `label_enum`.
    fn build_delegator_impl_get(&self) -> Tokens {
        let enum_ident = self.next_label.and_then(|m| m.get_enum_ident());
        let inner_root_name = Ident::new(
            &format!("{}Inner", &self.root_struct_name),
            Span::call_site(),
        );
        let metric_type = self.metric.metric_type.clone();

        if let Some(e) = enum_ident {
            let delegator_member_type = &self.delegator_member_type;
            let member_type = if self.is_secondary_last_label {
                if metric_type.to_string().contains("Counter") {
                    quote! {
                        AFLocalCounter<#inner_root_name,#metric_type,#delegator_member_type>
                    }
                } else {
                    quote! {
                        AFLocalHistogram<#inner_root_name,#delegator_member_type>
                    }
                }
            } else {
                delegator_member_type.into_token_stream()
            };

            let match_patterns = self
                .enum_definitions
                .get(e)
                .unwrap()
                .build_fields_with_path();
            let fields = self
                .next_label
                .expect("should have next label here")
                .get_value_def_list(self.enum_definitions)
                .get_names();
            quote! {
                pub fn get(&self, enum_value: #e) -> &#member_type {
                    match enum_value {
                        #(
                            #match_patterns => &self.#fields,
                        )*
                    }
                }
            }
        } else {
            Tokens::new()
        }
    }

    /// `fn get()` is only available when label is defined by `label_enum`.
    fn build_outer_impl_get(&self) -> Tokens {
        let enum_ident = self.label.get_enum_ident();
        if let Some(e) = enum_ident {
            let metric_type = self.metric.metric_type.clone();
            let inner_struct_name = self.inner_struct_name();
            let delegator_name = self.delegator_struct_name();

            let auto_flush_local = if metric_type.to_string().contains("Counter") {
                quote! {
                    AFLocalCounter<#inner_struct_name,#metric_type,#delegator_name>
                }
            } else {
                quote! {
                    AFLocalHistogram<#inner_struct_name,#delegator_name>
                }
            };

            let member_type = if self.metric.labels.len() == 1 {
                auto_flush_local
            } else {
                delegator_name.into_token_stream()
            };

            let match_patterns = self
                .enum_definitions
                .get(e)
                .unwrap()
                .build_fields_with_path();
            let fields = self
                .label
                .get_value_def_list(self.enum_definitions)
                .get_names();
            quote! {
                pub fn get(&self, enum_value: #e) -> &#member_type {
                    match enum_value {
                        #(
                            #match_patterns => &self.#fields,
                        )*
                    }
                }
            }
        } else {
            Tokens::new()
        }
    }

    fn build_inner_impl_from_body(&self, prev_labels_ident: &[Ident]) -> Tokens {
        let member_type = &self.inner_member_type;

        let init_instant = if self.label_index == 0 {
            quote! {
            last_flush: Cell::new(prometheus::timer::now_millis()),
            flush_millis: 1000,
            }
        } else {
            Tokens::new()
        };

        let bodies: Vec<_> = self
            .label
            .get_value_def_list(self.enum_definitions)
            .get()
            .iter()
            .map(|value| {
                let name = &value.name;
                let value = &value.value;
                if self.is_last_label {
                    let current_label = &self.label.label_key;
                    let prev_labels_str: Vec<_> = prev_labels_ident
                        .iter()
                        .enumerate()
                        .map(|(i, _)| &self.metric.labels[i].label_key)
                        .collect();
                    let local_suffix_call =
                        if util::is_local_metric(self.metric.metric_type.clone()) {
                            quote! { .local() }
                        } else {
                            Tokens::new()
                        };
                    quote! {
                        #name: m.with(&{
                            let mut coll = HashMap::new();
                            #(
                                coll.insert(#prev_labels_str, #prev_labels_ident);
                            )*
                            coll.insert(#current_label, #value);
                            coll
                        })#local_suffix_call,
                    }
                } else {
                    let prev_labels_ident = prev_labels_ident;
                    quote! {
                        #name: #member_type::from(
                            #(
                                #prev_labels_ident,
                            )*
                            #value,
                            m,
                        ),
                    }
                }
            })
            .collect();
        quote! {
            #(
                #bodies
            )*
            #init_instant
        }
    }

    fn build_inner_impl_flush(&self) -> Tokens {
        let value_def_list = self.label.get_value_def_list(self.enum_definitions);
        let names = value_def_list.get_names();
        quote! {
            pub fn flush(&self) {
                #(self.#names.flush();)*
            }
        }
    }

    fn build_with_flush_duration(&self) -> Tokens {
        if self.label_index == 0 {
            quote! {
                pub fn with_flush_duration(mut self, duration: std::time::Duration) -> Self {
                    self.flush_millis = prometheus::timer::duration_to_millis(duration);
                    self
                }
            }
        } else {
            Tokens::new()
        }
    }

    fn delegator_field_names(&self) -> Vec<Ident> {
        self.metric.labels[self.label_index + 1]
            .get_value_def_list(self.enum_definitions)
            .get_names()
            .iter()
            .map(|x| Ident::new(&x.to_string(), Span::call_site()))
            .collect()
    }

    fn build_delegator_struct(&self) -> Tokens {
        let struct_name = self.delegator_struct_name();
        let inner_root_name = Ident::new(
            &format!("{}Inner", &self.root_struct_name),
            Span::call_site(),
        );
        let metric_type = self.metric.metric_type.clone();
        let field_names = if self.is_last_label {
            (1..=self.metric.labels.len())
                .map(|suffix| Ident::new(&format!("offset{}", suffix), Span::call_site()))
                .collect::<Vec<Ident>>()
        } else {
            self.delegator_field_names()
        };

        let member_types = if self.is_last_label {
            (1..=self.metric.labels.len())
                .map(|_| self.delegator_member_type.clone().to_token_stream())
                .collect::<Vec<Tokens>>()
        } else if self.is_secondary_last_label {
            self.delegator_field_names()
                .iter()
                .map(|_| {
                    let delegator_member_type = self.delegator_member_type.clone();

                    if metric_type.to_string().contains("Counter") {
                        quote! {
                            AFLocalCounter<#inner_root_name,#metric_type,#delegator_member_type>
                        }
                    } else {
                        quote! {
                            AFLocalHistogram<#inner_root_name,#delegator_member_type>
                        }
                    }
                })
                .collect::<Vec<_>>()
        } else {
            self.delegator_field_names()
                .iter()
                .map(|_| {
                    let member_type = self.delegator_member_type.clone();
                    member_type.to_token_stream()
                })
                .collect::<Vec<_>>()
        };
        let root = if self.is_last_label {
            quote! {
                root: &'static LocalKey<#inner_root_name>,
            }
        } else {
            Tokens::new()
        };

        quote! {
            #[allow(missing_copy_implementations)]
            pub struct #struct_name {
                #root
                #(
                    pub #field_names: #member_types,
                )*
            }
        }
    }
}
