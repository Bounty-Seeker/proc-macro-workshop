//use proc_macro2::Ident;
use proc_macro2::TokenStream;
//use syn::parse::Parse;
use seq_inner_tokens::SeqTokens;
use syn::parse::ParseStream;
use syn::{Result};
use proc_macro2::Ident;

mod seq_inner_tokens;

#[derive(Debug)]
pub struct InnerSeqContent {
    contents: Vec<SeqTokens>,
    //ident : Ident
}

/*impl Parse for InnerSeqContent {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {

        let mut out_vec = Vec::new();

        while !input.is_empty() {
            let token : SeqTokens = input.parse()?;
            out_vec.push(token);
            println!("parsed so far {:?}\n\n\n", out_vec);
        }
        println!("parse created innerseqcontent");

        Ok(InnerSeqContent{
            contents : out_vec,
        })
    }
}*/


impl InnerSeqContent {

    pub fn create_parser(id : Ident) -> impl Fn(ParseStream<'_>) -> Result<Self> {

        let output_fn = move |input:ParseStream<'_>| {

            let mut out_vec = Vec::new();

            // get token parser
            let token_parser = SeqTokens::create_parser(id.clone());

            // go through stream and parse to token
            while !input.is_empty() {
                let token : SeqTokens = token_parser(input)?;
                out_vec.push(token);
                println!("parsed so far {:?}\n\n\n", out_vec);
            }
            println!("parse created innerseqcontent");

            Ok(InnerSeqContent{
                contents : out_vec,
                //ident: id.clone(),
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
        println!("started generating");
        let mut output = TokenStream::new();

        // for each value generate the token stream
        for token in &self.contents {
            let token_out = token.generate_output_repeated_whole(val, &ident_to_match)?;
            output.extend(token_out);
        }

        println!("generating {}", output);
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

    /*fn span(&self) -> Span {
        self.span
    }*/
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeqMode {
    Whole,
    Partial,
}
