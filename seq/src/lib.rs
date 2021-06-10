use proc_macro::TokenStream;
//use proc_macro2::{TokenStream as TS2};
use syn::parse_macro_input;
use seq_parsing::SeqParsed;

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let parsed_seq = parse_macro_input!(input as SeqParsed);



    let output = parsed_seq.create_tokenstream();
    //output.extend(parsed_seq.create_tokenstream());
    //dbg!("{:?}", &output);
    output.into()
}


mod seq_parsing;