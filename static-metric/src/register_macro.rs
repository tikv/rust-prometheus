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
use syn::synom::Synom;
use syn::*;

/// Matches `register_static_xxx_vec!(static_struct_name, metric_name, metric_desc, metric_labels)`
pub struct RegisterMethodInvoking {
    static_struct_name: Ident,
    metric_name: Expr,
    metric_desc: Expr,
    metric_labels: Expr,
}

impl Synom for RegisterMethodInvoking {
    named!(parse -> Self, do_parse!(
        static_struct_name: syn!(Ident) >>
        punct!(,) >>
        metric_name: syn!(Expr) >>
        punct!(,) >>
        metric_desc: syn!(Expr) >>
        punct!(,) >>
        metric_labels: syn!(Expr) >>
        option!(punct!(,)) >>
        (RegisterMethodInvoking {
            static_struct_name,
            metric_name,
            metric_desc,
            metric_labels,
        })
    ));
}

impl RegisterMethodInvoking {
    pub fn into_macro(self, register_type: &str) -> Tokens {
        let register_macro_name = Ident::from(format!("register_{}_vec", register_type));
        let (static_struct_name, metric_name, metric_desc, metric_labels) = (
            self.static_struct_name,
            self.metric_name,
            self.metric_desc,
            self.metric_labels,
        );
        quote!{
            {
                let metric_result = #register_macro_name!(
                    #metric_name,
                    #metric_desc,
                    #metric_labels
                );
                metric_result.map(|m| #static_struct_name::from(&m))
            }
        }
    }
}
