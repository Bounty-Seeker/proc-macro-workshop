use proc_macro::TokenStream;
use syn::{parse::Nothing, parse_macro_input, ItemStruct, ItemEnum};

mod bitfield_impl;
mod checks;
mod bitfield_specifier_impl;


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

#[proc_macro_derive(BitfieldSpecifier)]
pub fn derive(input: TokenStream) -> TokenStream {

        // parse input as ItemEnum
        let input_enum = parse_macro_input!(input as ItemEnum);

        // apply the macro
        let output = bitfield_specifier_impl::bitfield_specifier_impl(input_enum);
    
        // if output is error covert to compiler error and return
        output.unwrap_or_else(|err| err.to_compile_error()).into()
}


#[proc_macro]
pub fn generate_checks(input: TokenStream) -> TokenStream {
    
    // check no input
    parse_macro_input!(input as Nothing);

    // generate output
    let output = checks::create_checks_tokenstream();

    // if output is error covert to compiler error and return
    output.unwrap_or_else(|err| err.to_compile_error()).into()
}


