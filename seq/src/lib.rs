use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TS2};
use syn::parse_macro_input;
use seq_parsing::SeqParsed;

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let _parsed_seq = parse_macro_input!(input as SeqParsed);



    let output = TS2::new();
    //output.extend(_parsed_seq.contents);
    //dbg!("{:?}", &output);
    output.into()
}


mod seq_parsing {
use syn::{Ident, Token, braced, parse::Parse, token::Brace};
use proc_macro2::TokenStream;

    pub struct SeqParsed {
        pub initial_ident:Ident,
        pub in_token: Token![in],
        pub start_val : syn::LitInt,
        pub dot2 : Token![..],
        pub end_val : syn::LitInt,
        pub braced : Brace,
        pub contents: TokenStream,
    }

    impl Parse for SeqParsed {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let content;
        let parsed = Ok(SeqParsed{
            initial_ident: input.parse()?,
            in_token: input.parse()?,
            start_val:input.parse()?,
            dot2: input.parse()?,
            end_val:input.parse()?,
            braced: braced!(content in input),
            contents : content.parse()?,
        });
        dbg!("content: {:?}", content);
        parsed
    }
    }
}