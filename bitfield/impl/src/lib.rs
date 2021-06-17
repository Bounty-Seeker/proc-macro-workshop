use proc_macro::TokenStream;
use syn::{parse::Nothing, parse_macro_input, ItemStruct};

mod bitfield_impl;

#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {

    // check no args
    parse_macro_input!(args as Nothing);

    // parse input as ItemStruct
    let input_struct = parse_macro_input!(input as ItemStruct);

    // apply the macro
    let output = bitfield_impl::bitfield_impl(input_struct);

    // if output is error covert to compiler error and return
    output.unwrap_or_else(|err| err.to_compile_error()).into()
}


