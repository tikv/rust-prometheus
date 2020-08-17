// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use proc_macro2::{Span, TokenStream as Tokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::*;

/// Matches `register_static_xxx_vec!(static_struct_name, name, desc, labels, ...)`.
pub struct RegisterMethodInvoking {
    static_struct_name: Ident,
    arguments: Vec<Expr>,
}

impl Parse for RegisterMethodInvoking {
    fn parse(input: ParseStream) -> Result<Self> {
        let static_struct_name = input.parse()?;
        let _: Token![,] = input.parse()?;
        let p = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        let arguments = p.into_iter().collect();

        Ok(RegisterMethodInvoking {
            static_struct_name,
            arguments,
        })
    }
}

impl RegisterMethodInvoking {
    pub fn into_tokens(self, register_type: &str) -> Tokens {
        let register_macro_name = Ident::new(
            &format!("register_{}_vec", register_type),
            Span::call_site(),
        );
        let (static_struct_name, arguments) = (self.static_struct_name, self.arguments);
        quote! {
            {
                let metric_result = #register_macro_name!(#(#arguments),*);
                metric_result.map(|m| #static_struct_name::from(&m))
            }
        }
    }
}
