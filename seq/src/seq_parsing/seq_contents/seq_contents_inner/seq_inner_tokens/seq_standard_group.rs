use proc_macro2::{Delimiter, Span};

use crate::seq_parsing::seq_contents::seq_contents_inner::InnerSeqContent;
use proc_macro2:: Group;
use proc_macro2::{TokenStream, TokenTree};
// use syn::parse::Parse;
use syn::parse::{ParseStream, Parser};
use syn::{Result};
use proc_macro2::Ident;


use super::SeqMode;

#[derive(Debug)]
pub struct TTGroup {
    delimeter: Delimiter,
    group: InnerSeqContent,
    span: Span,
    //ident_to_match: Ident,
}

/*
impl Parse for TTGroup {

    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let tt_group : Group = input.parse()?;
        let delimeter = tt_group.delimiter();
        let span = tt_group.span();
        println!("parsing group {:?}", tt_group);

        let group_ts = tt_group.stream();

        let inner_content : InnerSeqContent = syn::parse2(group_ts)?;

        Ok(TTGroup{
            delimeter,
            group : inner_content,
            span,
        })
    }
}*/
/*
struct ParserCaller<F>
where F : Parser
{
    closure : Box<F>,
}

impl<F> ParserCaller<F>
where F:Parser<Output = InnerSeqContent> {
    fn new(closure:Box<F>) -> ParserCaller<F> {
        ParserCaller{
            closure
        }
    }
}

impl<F> Parser for ParserCaller<F>
where F : Parser<Output = InnerSeqContent>
{
    type Output = InnerSeqContent;

    fn parse2(self, tokens: TokenStream) -> syn::Result<InnerSeqContent> {
        let result: syn::Result<InnerSeqContent> = self.closure.parse2(tokens);
        result
    }
}*/

impl TTGroup {

    pub fn create_parser<'a>(id : Ident) -> impl Fn(ParseStream<'a>) -> Result<Self> {

        let output_fn = move |input:ParseStream<'_>| {

            let tt_group : Group = input.parse()?;
            let delimeter = tt_group.delimiter();
            let span = tt_group.span();
            println!("parsing group {:?}", tt_group);

            let group_ts = tt_group.stream();



            let inner_parser = InnerSeqContent::create_parser(id.clone());

            /*fn cast_parser<F: Fn(ParseStream) -> Result<InnerSeqContent>>(f: F) -> impl FnOnce(ParseStream) -> Result<InnerSeqContent> {
                move |stream | { f(stream) }
            }

            let inner_par = cast_parser(inner_parser);*/

            // impl parser trait for inner_parser hack

            let inner_content : InnerSeqContent = inner_parser.parse2(group_ts)?; // parse2(inner_parser,group_ts)?;

            Ok(TTGroup{
                delimeter,
                group : inner_content,
                span,
                //ident_to_match: id.clone(),
            })

        };

        output_fn
    }

    /// generate output token stream in case when we have only repeated groups
    pub fn generate_output_repeated_groups(
        &self,
        range: &std::ops::Range<u128>,
        ident_to_match: &Ident,
    ) -> syn::Result<TokenStream> {

        // generate tokenstream of tokenstream inside
        let internal_ts = self.group.generate_output_repeated_groups(range, ident_to_match)?;

        // create group
        let mut group = Group::new(self.delimeter, internal_ts);

        // set span
        group.set_span(self.span);

        // turn in tokentree
        let group_tt: TokenTree = group.into();

        Ok(group_tt.into())
    }


    /// generate output token stream in case when we are repeating whole contents
    pub fn generate_output_repeated_whole(
        &self,
        val: u128,
        ident_to_match: &Ident,
    ) -> syn::Result<TokenStream> {

        // generate tokenstream of tokenstream inside
        let internal_ts = self.group.generate_output_repeated_whole(val, ident_to_match)?;

        // create group
        let mut group = Group::new(self.delimeter, internal_ts);

        // set span
        group.set_span(self.span);

        // turn in tokentree
        let group_tt: TokenTree = group.into();

        Ok(group_tt.into())
    }


    /// validate that the tokenstream repeats all or only some sections
    pub fn validate(&self, cur_mode: &mut Option<SeqMode>) -> syn::Result<()> {
        self.group.validate(cur_mode)
    }
}

