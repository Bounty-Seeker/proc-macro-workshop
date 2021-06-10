use std::ops::Range;

use syn::{Ident, Token, braced, parse::Parse, token::{Brace}};
use proc_macro2::{Ident as Id, Literal, TokenStream, TokenTree, Group};

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
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let content;

        let initial_ident = input.parse()?;
        let in_token = input.parse()?;
        let start_val = input.parse()?;
        let dot2 = input.parse()?;
        let end_val = input.parse()?;
        let braced = braced!(content in input);
        let contents = content.parse()?;

        Ok(SeqParsed{
            initial_ident,
            in_token,
            start_val,
            dot2,
            end_val,
            braced,
            contents,
        })
    }
}

impl SeqParsed {

    /// create output tokenstream
    pub fn create_tokenstream(&self) -> TokenStream {

        let mut output = TokenStream::new();

        // get range
        let range = self.get_range();

        // check it parsed correctly
        if let Err(err) = range {
            return err.into_compile_error();
        }

        // Know it parsed so unwrap
        let range = range.unwrap();

        // for each value in the range output a clone of the token stream
        range.for_each(|val| {
            let ident = self.initial_ident.clone();
            let t_s = self.contents.clone();
            let altered_t_s = SeqParsed::alter_tokenstream(val, ident, t_s);
            output.extend(altered_t_s);
        });

        // give output
        output
    }


    // function that creates the altered tokenstream
    fn alter_tokenstream(val:u64, ident : Ident, t_s: TokenStream) -> TokenStream {

        // get ident in tokentree form
        let tt_ident : Id =
        if let TokenTree::Ident(ident) = ident.clone().into() {ident}
        else { unreachable!() };

        // get val in tokentree form
        let tt_val = TokenTree::Literal(Literal::u64_unsuffixed(val));

        let tt_iter = t_s.into_iter().map(|tt| {

            match tt {
                TokenTree::Group(group) => {

                    // group so turn into token stream and repeat

                    // group delimeter
                    let delimeter = group.delimiter();

                    // inner tokenstream
                    let in_t_s = group.stream();
                    let alter_in_t_s = SeqParsed::alter_tokenstream(val, ident.clone(), in_t_s);

                    // create new group and return as tree
                    Group::new(delimeter, alter_in_t_s).into()

                }
                TokenTree::Ident(ident) => {
                    // check ident matches given if true then return literal value
                    if ident == tt_ident {
                        tt_val.clone()
                    } else {
                        TokenTree::Ident(ident)
                    }
                }
                tt_pun @ TokenTree::Punct(_) => {
                    // return self
                    tt_pun
                },
                tt_lit @ TokenTree::Literal(_) => {
                    //return self
                    tt_lit
                },
            }

        });

        tt_iter.collect()
    }



    /// get range of repeat
    fn get_range(&self) -> syn::parse::Result<std::ops::Range<u64>> {

        let start = self.start_val.base10_parse()?;
        let end = self.end_val.base10_parse()?;
        let range = Range{start, end};
        Ok(range)
    }

}
