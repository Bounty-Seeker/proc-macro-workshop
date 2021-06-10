use std::{ops::Range};

use quote::format_ident;
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

        //println!("start :{:?} \n", t_s);
        // get ident in tokentree form
        let tt_ident : Id =
        if let TokenTree::Ident(ident) = ident.clone().into() {ident}
        else { unreachable!() };

        // get val in tokentree form
        let tt_val = TokenTree::Literal(Literal::u64_unsuffixed(val));

        let mut output = TokenStream::new();

        let mut tt_iter = t_s.into_iter().peekable();

        while let Some(tt) = tt_iter.next() {

        //.map(|tt| {

            match tt {
                TokenTree::Group(group) => {

                    // group so turn into token stream and repeat

                    // group delimeter and span
                    let delimeter = group.delimiter();
                    let span = group.span();

                    // inner tokenstream
                    let in_t_s = group.stream();
                    let alter_in_t_s = SeqParsed::alter_tokenstream(val, ident.clone(), in_t_s);

                    // create new group and add to output tokenstream
                    let mut alter_group : TokenTree = Group::new(delimeter, alter_in_t_s).into();
                    alter_group.set_span(span);
                    output.extend(TokenStream::from(alter_group));

                }

                TokenTree::Ident(ident) => {

                    let first_id = ident;

                    // check ident matches given if true then add literal value to output
                    if first_id == tt_ident {
                        output.extend(TokenStream::from(tt_val.clone()));
                        //println!("e");
                    }
                    else {
                        // check if the next token is punct
                        let next_tt = tt_iter.peek();

                        match next_tt {
                            Some(TokenTree::Punct(punc  )) => {

                                // i.e we have a punct next
                            
                                if  punc.as_char() == '#' {
                                
                                    //we have found hash
                                    let out_punc = TokenTree::Punct(punc.clone());
                                
                                    // consume hash to peek again
                                    tt_iter.next();
                                
                                    // get value after hash
                                    let nn_tt = tt_iter.peek();
                                    match nn_tt {
                                        Some(TokenTree::Ident(id)) => {
                                            // see if id matches given ident
                                            if tt_ident == *id {
                                                // have match so consume and add new ident to output
                                                tt_iter.next();

                                                let new_ident = format_ident!("{}{}", first_id,val);
                                                let out_id = TokenTree::Ident(new_ident);
                                                output.extend(TokenStream::from(out_id));
                                                //println!("a");


                                            } else {
                                                // consumed hash but no match so add first ident and hash to output
                                                let out_id = TokenTree::Ident(first_id);
                                                output.extend(TokenStream::from(out_id));

                                                output.extend(TokenStream::from(out_punc));
                                                //println!("b");

                                            }
                                        },
                                        _ => {
                                            // end of tokenstream or not a ident is next
                                            // consumed hash but no match so add first ident and hash to output
                                            let out_id = TokenTree::Ident(first_id);
                                            output.extend(TokenStream::from(out_id));

                                            output.extend(TokenStream::from(out_punc));
                                            //println!("c");

                                        },
                                    }
                                }
                                else {
                                // punc is next but is not a #
                                // no hash next so just add first ident
                                let out_id = TokenTree::Ident(first_id);
                                output.extend(TokenStream::from(out_id));
                                //println!("f");
                                }

                            },
                            _ => {
                                // end of tokenstream or not a punc is next
                                // no hash next so just add first ident
                                let out_id = TokenTree::Ident(first_id);
                                output.extend(TokenStream::from(out_id));
                                //println!("d");

                            },
                        }
                    }

                }

                TokenTree::Punct(punc) => {
                    // see if punc is # if it is check next element is given ident and change it
                    let punc_char = punc.as_char();

                    //dbg!("char: {}", &punc_char);
                    if punc_char != '#' {
                        let tt_punc = TokenTree::Punct(punc);
                        output.extend(TokenStream::from(tt_punc));
                        continue;
                    }

                    // from here know have #

                    // get peeker
                    let next_tt = tt_iter.peek();

                    if next_tt.is_none() {
                        // end of stream so add # to output
                        let tt_punc = TokenTree::Punct(punc);
                        output.extend(TokenStream::from(tt_punc));
                        continue;
                    }

                    let next_tt = next_tt.unwrap();

                    if let TokenTree::Ident(next_id) = next_tt {
                        // got ident next

                        if *next_id == tt_ident {
                            // got matching ident

                            // add value literal to output
                            let tt_val: TokenTree = tt_val.clone();
                            output.extend(TokenStream::from(tt_val));

                            // consume the ident in main iter
                            tt_iter.next();

                        } else {
                            // not matching ident so put punct on output
                            let tt_punc = TokenTree::Punct(punc);
                            output.extend(TokenStream::from(tt_punc));
                            continue;
                        }

                    } else {
                        // not a ident so add punct and continue
                        let tt_punc = TokenTree::Punct(punc);
                        output.extend(TokenStream::from(tt_punc));
                        continue;
                    }

                },
                tt_lit @ TokenTree::Literal(_) => {
                    // add self to output
                    output.extend(TokenStream::from(tt_lit));
                },
            }

            }

        //println!("finish: {:?} \n\n\n\n", output);

        output
    }


    /// get range of repeat
    fn get_range(&self) -> syn::parse::Result<std::ops::Range<u64>> {

        let start = self.start_val.base10_parse()?;
        let end = self.end_val.base10_parse()?;
        let range = Range{start, end};
        Ok(range)
    }

}
