use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {

    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);


    let output = TS2::new();
    output.into()
}
