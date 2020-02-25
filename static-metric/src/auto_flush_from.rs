use proc_macro2::Span;
use syn::export::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::token::*;
use syn::*;

pub struct AutoFlushFromDef {
    class_name: Ident,
    inner_class_name: Ident,
    source_var_name: Ident,
    target_var_name: Ident,
}

impl Parse for AutoFlushFromDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let target_var_name: Ident = input.parse()?;
        let _: Colon = input.parse()?;
        let class_name: Ident = input.parse()?;
        let _: Eq = input.parse()?;
        let source_var_name: Ident = input.parse()?;
        let inner_class_name = Ident::new(&format!("{}Inner", class_name), Span::call_site());

        Ok(AutoFlushFromDef {
            target_var_name,
            class_name,
            inner_class_name,
            source_var_name,
        })
    }
}

impl AutoFlushFromDef {
    pub fn auto_flush_from(&self) -> TokenStream2 {
        let target_var_name = self.target_var_name.clone();
        let class_name = self.class_name.clone();
        let inner_class_name = self.inner_class_name.clone();
        let var_name = self.source_var_name.clone();
        quote! {
            lazy_static!{
                pub static ref #target_var_name: #class_name = {
                    thread_local! {
                      pub static INNER: #inner_class_name = #inner_class_name::from(& #var_name);
                    }
                    #class_name::from(&INNER)
                };
            }
        }
    }
}
