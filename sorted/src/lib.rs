use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::ToTokens;
use syn::{parse_macro_input, Error, Item, ItemFn};

mod enum_sort;
mod match_sort;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    // Check no args
    if !args.is_empty() {
        let err = Error::new_spanned(TS2::from(args), "Unexpected args")
            .into_compile_error()
            .into();
        return err;
    }

    // Parse input as syn::item
    let input_item: Item = parse_macro_input!(input as Item);

    // enum_sorted
    let is_sorted = enum_sort::sorted_macro(&input_item);

    // Create output
    let mut output: TS2 = input_item.into_token_stream();

    match is_sorted {
        Ok(()) => {}
        Err(err) => output.extend(err.into_compile_error()),
    }

    output.into()
}

#[proc_macro_attribute]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    // Check no args
    if !args.is_empty() {
        let err = Error::new_spanned(TS2::from(args), "Unexpected args")
            .into_compile_error()
            .into();
        return err;
    }

    // Parse input as syn::item
    let mut input_item: ItemFn = parse_macro_input!(input as ItemFn);

    // run core of sorted macro i.e. check sorted
    let is_sorted = match_sort::sorted_check_macro(&mut input_item);

    // Create output
    let mut output: TS2 = input_item.into_token_stream();

    // turn into suitable output types
    match is_sorted {
        Ok(()) => {}
        Err(err) => output.extend(err.into_compile_error()),
    }

    output.into()
}
