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

use quote::Tokens;
use syn::punctuated::Punctuated;
use syn::synom::Synom;
use syn::*;

/// Matches `register_static_xxx_vec!(static_struct_name, name, desc, labels, ...)`.
pub struct RegisterMethodInvoking {
    static_struct_name: Ident,
    arguments: Vec<Expr>,
}

impl Synom for RegisterMethodInvoking {
    named!(parse -> Self, do_parse!(
        static_struct_name: syn!(Ident) >>
        punct!(,) >>
        arguments: call!(Punctuated::<Expr, Token![,]>::parse_terminated_nonempty) >>
        (RegisterMethodInvoking {
            static_struct_name,
            arguments: arguments.into_iter().collect(),
        })
    ));
}

impl RegisterMethodInvoking {
    pub fn into_tokens(self, register_type: &str) -> Tokens {
        let register_macro_name = Ident::from(format!("register_{}_vec", register_type));
        let (static_struct_name, arguments) = (self.static_struct_name, self.arguments);
        quote! {
            {
                let metric_result = #register_macro_name!(#(#arguments),*);
                metric_result.map(|m| #static_struct_name::from(&m))
            }
        }
    }
}
