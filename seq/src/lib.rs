use proc_macro::TokenStream;
//use proc_macro2::{TokenStream as TS2};
use seq_parsing::SeqOuter;
use syn::parse_macro_input;

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    // parse input
    let parsed_seq = parse_macro_input!(input as SeqOuter);

    // get output
    let output = parsed_seq.create_tokenstream();

    // if output is error covert to compiler error and return
    output.unwrap_or_else(|err| err.to_compile_error()).into()
}

mod seq_parsing;
