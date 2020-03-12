use proc_macro2::Span;
use syn::export::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::token::*;
use syn::*;

pub struct AutoFlushFromDef {
    class_name: Ident,
    inner_class_name: Ident,
    source_var_name: Expr,
    flush_duration: Option<Expr>,
}

impl Parse for AutoFlushFromDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let source_var_name: Expr = input.parse()?;
        let _: Comma = input.parse()?;
        let class_name: Ident = input.parse()?;
        let inner_class_name = Ident::new(&format!("{}Inner", class_name), Span::call_site());

        let flush_duration = if input.peek(Comma) {
            let _: Comma = input.parse()?;
            let res: Expr = input.parse()?;
            Some(res)
        } else {
            None
        };

        Ok(AutoFlushFromDef {
            class_name,
            inner_class_name,
            source_var_name,
            flush_duration,
        })
    }
}

impl AutoFlushFromDef {
    pub fn auto_flush_from(&self) -> TokenStream2 {
        let inner_class_name = self.inner_class_name.clone();
        let class_name = self.class_name.clone();
        let source_var_name = self.source_var_name.clone();
        let update_duration = match &self.flush_duration {
            Some(d) => {
                quote! {
                    .with_flush_duration(#d.into())
                }
            }
            None => quote! {},
        };
        quote! {
            {
                thread_local! {
                    static INNER: #inner_class_name = #inner_class_name::from(& #source_var_name)#update_duration;
                }
                #class_name::from(&INNER)
            }
        }
    }
}
