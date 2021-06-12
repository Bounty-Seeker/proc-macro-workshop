use super::{SeqMode};
use proc_macro2::Ident;
use proc_macro2::{TokenStream};
use syn::parse::ParseStream;
use syn::token::{Paren};
use syn::{Error, Token, parenthesized, Result};
use super::super::InnerSeqContent;

#[derive(Debug)]
pub struct RepeatedGroup {
    hash: Token!(#),
    _paren : Paren,
    contents: InnerSeqContent,
    _ask : Token!(*),
}

/*
impl Parse for RepeatedGroup {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let hash : Token!(#) = input.parse()?;
        let content;
        let paren =  parenthesized!(content in input);
        let contents : InnerSeqContent = content.parse()?;
        let ask : Token!(*) = input.parse()?;

        Ok(RepeatedGroup{
            hash,
            _paren:paren,
            contents,
            _ask:ask,
        })
    }
}*/

impl RepeatedGroup {

    pub fn create_parser<'a>(id : Ident) -> impl Fn(ParseStream<'a>) -> Result<Self> {

        let output_fn = move |input:ParseStream<'_>| {

            let hash : Token!(#) = input.parse()?;
            let content;
            let paren =  parenthesized!(content in input);
            let contents_parser = InnerSeqContent::create_parser(id.clone());
            let contents : InnerSeqContent = contents_parser(&content)?;
            let ask : Token!(*) = input.parse()?;

            Ok(RepeatedGroup{
                hash,
                _paren:paren,
                contents,
                _ask:ask,
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

        let mut output = TokenStream::new();

        // generate tokenstream for each val and token
        for val in range.clone() {
            let out_val = self.contents.generate_output_repeated_whole(val, ident_to_match)?;
            output.extend(out_val);
        }

        Ok(output)
    }

    /// validate that the tokenstream repeats all or only some sections
    pub fn validate(&self, cur_mode: &mut Option<SeqMode>) -> syn::Result<()> {

        match *cur_mode {
            Some(SeqMode::Partial) => {
                // already found partial mode

                // need to validate that the internals of this behave as if repeating whole
                let mut inner_mode = Some(SeqMode::Whole);
                self.contents.validate(&mut inner_mode)?;
                Ok(())
            },
            Some(SeqMode::Whole) => {
                // previously found Whole mode. This means error
                let err_msg = "Invalid syntax. Must repeat whole or just parts";
                let err = Error::new(self.hash.span,err_msg);
                Err(err)
            },
            None => {
                // haven't found previous anything to determine which mode
                // As repeated group so set mode to partial
                *cur_mode = Some(SeqMode::Partial);

                // need to validate that the internals of this behave as if repeating whole
                let mut inner_mode = Some(SeqMode::Whole);
                self.contents.validate(&mut inner_mode)?;

                Ok(())
            },
        }
    }
}
