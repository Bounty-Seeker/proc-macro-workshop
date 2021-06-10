use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TS2};
use syn::parse_macro_input;
use seq_parsing::SeqParsed;

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let _parsed_seq = parse_macro_input!(input as SeqParsed);



    let output = TS2::new();
    output.into()
}


mod seq_parsing {
use syn::{Ident, Token, braced, parse::Parse, token::Brace};

    pub struct SeqParsed {
        initial_ident:Ident,
        in_token: Token![in],
        start_val : syn::LitInt,
        dot2 : Token![..],
        end_val : syn::LitInt,
        braced : Brace,
    }

    impl Parse for SeqParsed {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let _content;
        Ok(SeqParsed{
            initial_ident: input.parse()?,
            in_token: input.parse()?,
            start_val:input.parse()?,
            dot2: input.parse()?,
            end_val:input.parse()?,
            braced: braced!(_content in input),
        })
    }
    }
}