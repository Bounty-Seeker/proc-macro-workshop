use proc_macro2::TokenStream;
use seq_inner_tokens::SeqTokens;
use syn::parse::ParseStream;
use syn::{Result};
use proc_macro2::Ident;

mod seq_inner_tokens;

#[derive(Debug)]
pub struct InnerSeqContent {
    contents: Vec<SeqTokens>,
}

impl InnerSeqContent {

    pub fn create_parser(id : Ident) -> impl Fn(ParseStream<'_>) -> Result<Self> {

        move |input:ParseStream<'_>| {

            let mut out_vec = Vec::new();

            // get token parser
            let token_parser = SeqTokens::create_parser(id.clone());

            // go through stream and parse to token
            while !input.is_empty() {
                let token : SeqTokens = token_parser(input)?;
                out_vec.push(token);
            }

            Ok(InnerSeqContent{
                contents : out_vec,
            })

        }
    }

    /// generate output token stream in case when we have only repeated groups
    pub fn generate_output_repeated_groups(
        &self,
        range: &std::ops::Range<u128>,
        ident_to_match: &Ident,
    ) -> syn::Result<TokenStream> {
        let mut output = TokenStream::new();

        // for each token generate output
        for token in &self.contents {
            let token_out = token.generate_output_repeated_groups(&range, &ident_to_match)?;
            output.extend(token_out);
        }

        Ok(output)
    }


    /// generate output token stream in case when we are repeating whole contents
    pub fn generate_output_repeated_whole(
        &self,
        val: u128,
        ident_to_match: &Ident,
    ) -> syn::Result<TokenStream> {
        //println!("started generating");
        let mut output = TokenStream::new();

        // for each value generate the token stream
        for token in &self.contents {
            let token_out = token.generate_output_repeated_whole(val, &ident_to_match)?;
            output.extend(token_out);
        }

        //println!("generating {}", output);
        Ok(output)
    }


    /// validate that the tokenstream repeats all r only some sections
    pub fn validate(&self,  cur_mode: &mut Option<SeqMode>)  -> syn::Result<()> {

        // for each token validate
        for token in &self.contents {
            token.validate(cur_mode)?;
        }

        // return mode
        Ok(())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeqMode {
    Whole,
    Partial,
}
