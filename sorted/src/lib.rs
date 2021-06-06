use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::ToTokens;
use syn::{Item, parse_macro_input};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    //let _ = args;

    // Check no args
    if !args.is_empty() {
        panic!()
    }

    // Parse input as syn::item
    let input_item : Item = parse_macro_input!(input as Item);


    //println!("{:?}", args);
    //println!("{:?}", input_item);


    let mut output = TS2::new();
    output.extend(input_item.into_token_stream());
    output.into()
}
