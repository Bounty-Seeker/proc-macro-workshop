use proc_macro2::Ident;
use syn::parse::ParseStream;
use syn::{Result, Error};
use super::SeqMode;
use proc_macro2::{TokenStream, TokenTree, Literal};

#[derive(Debug)]
pub struct MatIdent{
    ident : Ident
}


impl MatIdent {

    pub fn create_parser<'a>(id : Ident) -> impl Fn(ParseStream<'a>) -> Result<Self> {

        move |input:ParseStream<'_>| {
            let parsed_id : Ident = input.parse()?;

            if parsed_id == id {
                Ok(MatIdent{
                    ident:parsed_id,
                })
            } else {
                let err = input.error("ident doesn't match");
                Err(err)
            }

        }
    }


    /// generate output token stream in case when we are repeating whole contents
    pub fn generate_output_repeated_whole(
        &self,
        val: u128,
        _ident_to_match: &Ident,
    ) -> syn::Result<TokenStream> {

        let tt_lit: TokenTree = Literal::u128_unsuffixed(val).into();

        Ok(tt_lit.into())

    }


    /// validate that the tokenstream repeats all or only some sections
    pub fn validate(&self, cur_mode: &mut Option<SeqMode>) -> syn::Result<()> {
        
        match *cur_mode {
            Some(SeqMode::Partial) => {
                // previously found Partial mode. This means error
                let err_msg = "Invalid syntax. Must repeat whole or just parts";
                let err = Error::new(self.ident.span(),err_msg);
                Err(err)
            },
            Some(SeqMode::Whole) => {
                // already found Whole mode
                Ok(())
            },
            None => {
                // haven't found previous anything to determine which mode
                // As matching ident so set mode to Whole
                *cur_mode = Some(SeqMode::Whole);
                Ok(())
            },
        }

    }
}
