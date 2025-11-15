use crate::{error::MacroResult, input::EndpointDef};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::method::{FnNameExpander, ParamsExpander};

pub struct TraitExpander<'a> {
    endpoints: &'a [EndpointDef],
    trait_name: &'a Ident,
    error_name: &'a Ident,
}

impl<'a> TraitExpander<'a> {
    pub fn new(endpoints: &'a [EndpointDef], trait_name: &'a Ident, error_name: &'a Ident) -> Self {
        Self {
            endpoints,
            trait_name,
            error_name,
        }
    }

    pub fn expand(&self) -> MacroResult<TokenStream> {
        let trait_name = self.trait_name;
        let trait_methods = self.expand_trait_methods();

        Ok(quote! {
            pub trait #trait_name {
                #(#trait_methods)*
            }
        })
    }

    fn expand_trait_methods(&self) -> Vec<TokenStream> {
        self.endpoints
            .iter()
            .map(|def| {
                let fn_name = FnNameExpander::new(def).expand();
                let params = ParamsExpander::new(def).expand();
                let res = def
                    .res
                    .as_ref()
                    .map(|t| quote! { #t })
                    .unwrap_or_else(|| quote! { () });
                let error_name = self.error_name;

                quote! {
                    async fn #fn_name(&self, #(#params),*) -> Result<#res, #error_name>;
                }
            })
            .collect()
    }
}
