use super::SeqMode;
use proc_macro2::Ident;
use proc_macro2::{TokenStream, TokenTree};
use syn::Token;
use syn::parse::ParseStream;
use syn::{Error, Result};


#[derive(Debug)]
pub struct AlteredIdent {
    ident_before_hash: Ident,
    _hash : Token!(#),
    ident_after_hash: Ident,
}

impl AlteredIdent {

    pub fn create_parser<'a>(id : Ident) -> impl Fn(ParseStream<'a>) -> Result<Self> {

        move |input:ParseStream<'_>| {

            let ident_before_hash : Ident = input.parse()?;
            let hash : Token!(#) = input.parse()?;
            let ident_after_hash : Ident = input.parse()?;

            if ident_after_hash != id.clone() {
                let err_span = ident_before_hash.span();
                let err_msg = "ident after hash didn't match given ident";
                let err = Error::new(err_span, err_msg);
                return Err(err);
            }
    
            Ok(AlteredIdent{
                ident_before_hash,
                _hash:hash,
                ident_after_hash,
            })

        }
    }

    /// generate output token stream in case when we are repeating whole contents
    pub fn generate_output_repeated_whole(
        &self,
        val: u128,
        ident_to_match: &Ident,
    ) -> syn::Result<TokenStream> {

        // check second ident is the same
        if *ident_to_match != self.ident_after_hash {
            let err_span = self.ident_after_hash.span();
            let err_msg = format!("expected {} found {} after #", ident_to_match, self.ident_after_hash);
            let err = Error::new(err_span,err_msg);
            return Err(err);
        }

        //create new ident
        let new_id = format!("{}{}",self.ident_before_hash, val);
        let new_ident = Ident::new(new_id.as_str(), self.ident_before_hash.span());

        let new_tt: TokenTree = new_ident.into();

        Ok(new_tt.into())
    }


    /// validate that the tokenstream repeats all r only some sections
    pub fn validate(&self, cur_mode: &mut Option<SeqMode>) -> syn::Result<()> {

        match *cur_mode {
            Some(SeqMode::Whole) => {
                // already found whole mode
                Ok(())
            },
            Some(SeqMode::Partial) => {
                // previously found Partial mode. This means error
                let err_msg = "Invalid syntax. Must repeat whole or just parts";
                let err = Error::new(self.ident_before_hash.span(),err_msg);
                Err(err)
            },
            None => {
                // haven't found previous anything to determine which mode
                // As altered ident so set mode to Whole
                *cur_mode = Some(SeqMode::Whole);
                Ok(())
            },
        }
    }

}
