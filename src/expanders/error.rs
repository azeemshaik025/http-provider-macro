use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub struct ErrorExpander<'a> {
    error_name: &'a Ident,
}

impl<'a> ErrorExpander<'a> {
    pub fn new(error_name: &'a Ident) -> Self {
        Self { error_name }
    }

    pub fn expand(&self) -> TokenStream {
        let error_name = self.error_name;

        quote! {
            #[derive(Debug)]
            pub enum #error_name {
                UrlConstruction(String),
                Request(reqwest::Error),
                Http { status: u16, reason: String },
                Deserialization(String),
            }

            impl std::fmt::Display for #error_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        Self::UrlConstruction(msg) => write!(f, "Failed to construct URL: {}", msg),
                        Self::Request(err) => write!(f, "Request failed: {}", err),
                        Self::Http { status, reason } => write!(f, "HTTP {} {}", status, reason),
                        Self::Deserialization(msg) => write!(f, "Failed to deserialize: {}", msg),
                    }
                }
            }

            impl std::error::Error for #error_name {
                fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                    match self {
                        Self::Request(err) => Some(err),
                        _ => None,
                    }
                }
            }

            impl From<reqwest::Error> for #error_name {
                fn from(err: reqwest::Error) -> Self {
                    Self::Request(err)
                }
            }
        }
    }
}
