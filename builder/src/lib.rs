use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::{quote, format_ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // TokenStream::new()

    // get ident of given type
    let identi = input.ident;

    // create ident of our builder
    let builder_ident = format_ident!("{}Builder", identi);

    // create the builder struct and functions
    let output = quote!(
        pub struct #builder_ident {
        executable: Option<String>,
        args: Option<Vec<String>>,
        env: Option<Vec<String>>,
        current_dir: Option<String>,
        }

        impl #identi {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        });

    TokenStream::from(output)
}