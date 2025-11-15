use crate::{
    error::{MacroError, MacroResult},
    input::HttpProviderInput,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub mod error;
pub mod interface;
pub mod method;

pub use error::ErrorExpander;
pub use interface::TraitExpander;
pub use method::MethodExpander;

pub struct HttpProviderExpander {
    input: HttpProviderInput,
}

impl HttpProviderExpander {
    pub fn new(input: HttpProviderInput) -> Self {
        Self { input }
    }

    pub fn expand(&self) -> MacroResult<TokenStream> {
        self.validate()?;

        let struct_name = &self.input.struct_name;
        let error_name = Ident::new(&format!("{}Error", struct_name), struct_name.span());

        let error_type = ErrorExpander::new(&error_name).expand();
        let trait_def = self.expand_trait_def(&error_name)?;
        let methods = self.expand_methods(&error_name)?;
        let struct_impl = self.expand_struct_impl(&methods);

        Ok(quote! {
            #error_type
            #trait_def
            #struct_impl
        })
    }

    fn expand_trait_def(&self, error_name: &Ident) -> MacroResult<TokenStream> {
        let trait_name = self.trait_name();
        TraitExpander::new(&self.input.endpoints, &trait_name, &error_name).expand()
    }

    fn expand_methods(&self, error_name: &Ident) -> MacroResult<Vec<TokenStream>> {
        self.input
            .endpoints
            .iter()
            .map(|def| MethodExpander::new(def, error_name).expand())
            .collect()
    }

    fn expand_struct_impl(&self, methods: &[TokenStream]) -> TokenStream {
        let struct_name = &self.input.struct_name;
        let trait_name = self.trait_name();
        quote! {
            pub struct #struct_name {
                url: reqwest::Url,
                client: reqwest::Client,
                timeout: std::time::Duration,
            }

            impl #struct_name {
                pub fn new(url: reqwest::Url, timeout: Option<u64>) -> Self {
                    let client = reqwest::Client::new();
                    let timeout = std::time::Duration::from_millis(timeout.unwrap_or(5000));
                    Self { url, client, timeout }
                }
            }

            impl #trait_name for #struct_name {
                #(#methods)*
            }
        }
    }

    fn trait_name(&self) -> Ident {
        Ident::new(
            &format!("{}Trait", self.input.struct_name),
            self.input.struct_name.span(),
        )
    }

    fn validate(&self) -> MacroResult<()> {
        if self.input.endpoints.is_empty() {
            return Err(MacroError::NoEndpointsConfigured {
                span: self.input.struct_name.span(),
            });
        }
        Ok(())
    }
}
