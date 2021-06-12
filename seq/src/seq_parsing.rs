use std::ops::Range;
use seq_contents::ValidatedSeqContents;

use proc_macro2::TokenStream;
use syn::{braced, parse::Parse, token::Brace, Ident, Result, Token};

mod seq_contents;

pub struct SeqOuter {
    pub initial_ident: Ident,
    pub in_token: Token![in],
    pub start_val: syn::LitInt,
    pub dot2: Token![..],
    pub equal : Option<Token!(=)>,
    pub end_val: syn::LitInt,
    pub braced: Brace,
    pub contents: ValidatedSeqContents,
}

impl Parse for SeqOuter {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let content;

        let initial_ident: Ident = input.parse()?;
        let in_token = input.parse()?;
        let start_val = input.parse()?;
        let dot2 = input.parse()?;

        let equal;
        if  input.peek(Token!(=)) {
            equal = Some(input.parse()?)
        }else {
            equal = None;
        }
        let end_val = input.parse()?;
        let braced = braced!(content in input);
        let cloned_ident : Ident = initial_ident.clone();
        let content_parser = ValidatedSeqContents::create_parser(cloned_ident);
        let contents = content_parser(&content)?;

        Ok(SeqOuter {
            initial_ident,
            in_token,
            start_val,
            dot2,
            equal,
            end_val,
            braced,
            contents,
        })
    }
}

impl SeqOuter {
    /// create output tokenstream
    pub fn create_tokenstream(self) -> Result<TokenStream> {
        // get range
        let range = self.get_range()?;

        // get output
        self.contents.generate_output(range, self.initial_ident)
    }

    /// get range of repeat
    fn get_range(&self) -> syn::parse::Result<std::ops::Range<u128>> {
        let start = self.start_val.base10_parse()?;
        let mut end = self.end_val.base10_parse()?;
        if self.equal.is_some() {
            if end == u128::MAX { panic!("can't use u128::MAX as upper limit when inclusive") }
            end += 1;
        }
        let range = Range { start, end };
        Ok(range)
    }
}
