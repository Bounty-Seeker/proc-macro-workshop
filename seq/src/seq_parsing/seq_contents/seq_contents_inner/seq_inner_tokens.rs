use std::u128;

use super::SeqMode;
use proc_macro2::{Ident, Literal, Punct};
use proc_macro2::{TokenStream, TokenTree};
use seq_standard_group::TTGroup;
//use syn::parse::Parse;
use seq_altered_ident::AlteredIdent;
use seq_repeated_group::RepeatedGroup;
use seq_matching_ident::MatIdent;
use seq_my_ident::MyIdent;

use syn::parse::ParseStream;
use syn::Result;

mod seq_altered_ident;
mod seq_repeated_group;
mod seq_standard_group;
mod seq_matching_ident;
mod seq_my_ident {
    use proc_macro2::Ident;
    use quote::ToTokens;
    use syn::{ext::IdentExt, parse::Parse};

    #[derive(Debug)]
    pub struct MyIdent {
        ident:Ident
    }

    impl Parse for MyIdent {

        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

            // println!("Trying parse as ident");

            let ident = input.call(syn::Ident::parse_any)?;

            // println!("Found ident {:?}", ident);
            Ok(MyIdent{
                ident
            })
        }
    }

    impl MyIdent {
        /// generate output token stream in case when we have only repeated groups
        pub fn generate_output_repeated_groups(
            &self,
            _range: &std::ops::Range<u128>,
            _ident_to_match: &Ident,
        ) -> syn::Result<proc_macro2::TokenStream> {

            Ok(self.ident.to_token_stream())
        }

        /// generate output token stream in case when we are repeating whole contents
        pub fn generate_output_repeated_whole(
            &self,
            _val: u128,
            _ident_to_match: &Ident,
        ) -> syn::Result<proc_macro2::TokenStream> {

            Ok(self.ident.to_token_stream())
        }
    }
}

#[derive(Debug)]
pub enum SeqTokens {
    TTPunc(Punct),
    TTIden(MyIdent),
    TTLit(Literal),
    TTMatIden(MatIdent),
    TTGroup(TTGroup),
    RepeatedGroup(RepeatedGroup),
    AlteredIdent(AlteredIdent),
}


impl SeqTokens {

    pub fn create_parser<'a>(id : Ident) -> impl Fn(ParseStream<'a>) -> Result<Self> {

        move |input : ParseStream<'_>| {

            //println!("Token parsing seqtokens {:?}", input);

            //println!("try parse alt id");

            // try parse as altered ident
            let forked_buf = input.fork();
            let alt_ident_parser = AlteredIdent::create_parser(id.clone());
            if alt_ident_parser(&forked_buf).is_ok() {
                let alt_id = alt_ident_parser(input).map(SeqTokens::AlteredIdent);
                //println!("parse alt id");
                return alt_id;
            }
    
            //println!("try parse rep group id");
            // try parse as repeated group
            let forked_buf = input.fork();
            let rep_group_parser = RepeatedGroup::create_parser(id.clone());
            if rep_group_parser(&forked_buf).is_ok() {
                let rep_group = rep_group_parser(input).map(SeqTokens::RepeatedGroup);
                //println!("parse rep group");
                return rep_group;
            }

            //println!("try parse matching id");

            // try parse as matching ident
            let forked_buf = input.fork();
            let match_id_parser = MatIdent::create_parser(id.clone());
            if match_id_parser(&forked_buf).is_ok() {
                let match_id = match_id_parser(input).map(SeqTokens::TTMatIden);
                //println!("parse match id");
                return match_id;
            }


            //println!("try parse std group");
            // try parse as standard group
            let forked_buf = input.fork();
            let std_group_parser = TTGroup::create_parser(id.clone());
            if std_group_parser(&forked_buf).is_ok() {
                let std_group = std_group_parser(input).map(SeqTokens::TTGroup);
                //println!("parse std group");
                return std_group;
            }
    
            //println!("try parse std id");
            // try parse as standard ident
            if input.fork().parse::<MyIdent>().is_ok() {
                let id = input.parse().map(SeqTokens::TTIden);
                //println!("parse std id");
                return id;
            }
    
            //println!("try parse punc");

            // try parse as punc
            if input.fork().parse::<Punct>().is_ok() {
                let punc = input.parse().map(SeqTokens::TTPunc);
                //println!("parse punc");
                return punc;
            }
    
    
            //println!("try parse lit");

            // try parse as lit
            if input.fork().parse::<Literal>().is_ok() {
                let lit = input.parse().map(SeqTokens::TTLit);
                //println!("parse lit");
                return lit;
            }
    
            //println!("parse failed");
    
            let err = input.error("Expected a SeqToken");
            Err(err)
        }

    }

    /// generate output token stream in case when we have only repeated groups
    pub fn generate_output_repeated_groups(
        &self,
        range: &std::ops::Range<u128>,
        ident_to_match: &Ident,
    ) -> syn::Result<TokenStream> {

        match self {
            SeqTokens::TTPunc(tt_punc) => {
                let punc_tt : TokenTree = tt_punc.clone().into();
                Ok(punc_tt.into())
            },
            SeqTokens::TTIden(my_ident) => {
                let ident_ts = my_ident.generate_output_repeated_groups(range, ident_to_match)?;
                Ok(ident_ts)
            },
            SeqTokens::TTLit(tt_lit) => {
                let lit_tt : TokenTree = tt_lit.clone().into();
                Ok(lit_tt.into())
            },
            SeqTokens::TTGroup(tt_group) => {
                let group_ts = tt_group.generate_output_repeated_groups(range, ident_to_match)?;
                Ok(group_ts)
            },
            SeqTokens::RepeatedGroup(rep_group) => {
                let group_ts = rep_group.generate_output_repeated_groups(range, ident_to_match)?;
                Ok(group_ts)
            },
            SeqTokens::AlteredIdent(_) => {
                unreachable!()
            },
            SeqTokens::TTMatIden(_) => {
                unreachable!()
            },
        }
    }


    /// generate output token stream in case when we are repeating whole contents
    pub fn generate_output_repeated_whole(
        &self,
        val: u128,
        ident_to_match: &Ident,
    ) -> syn::Result<TokenStream> {

        match self {
            SeqTokens::TTPunc(tt_punc) => {
                let punc_tt : TokenTree = tt_punc.clone().into();
                Ok(punc_tt.into())
            },
            SeqTokens::TTIden(my_ident) => {
                let ident_ts = my_ident.generate_output_repeated_whole(val, ident_to_match)?;
                Ok(ident_ts)
            },
            SeqTokens::TTLit(tt_lit) => {
                let lit_tt : TokenTree = tt_lit.clone().into();
                Ok(lit_tt.into())
            },
            SeqTokens::TTGroup(tt_group) => {
                let group_ts = tt_group.generate_output_repeated_whole(val, ident_to_match)?;
                Ok(group_ts)
            },
            SeqTokens::RepeatedGroup(_) => {
                unreachable!();
            },
            SeqTokens::AlteredIdent(alt_id) => {
                let tt_ident = alt_id.generate_output_repeated_whole(val, ident_to_match)?;
                Ok(tt_ident)
            },
            SeqTokens::TTMatIden(mat_id) => {
                let tt_mat = mat_id.generate_output_repeated_whole(val, ident_to_match)?;
                Ok(tt_mat)
            },
        }

    }


    /// validate that the tokenstream repeats all r only some sections
    pub fn validate(&self, cur_mode: &mut Option<SeqMode>) -> syn::Result<()> {

        match self {
            SeqTokens::TTPunc(_) => Ok(()),
            SeqTokens::TTIden(_) => Ok(()),
            SeqTokens::TTLit(_) => Ok(()),
            SeqTokens::TTGroup(tt_group) => {
                tt_group.validate(cur_mode)
            },
            SeqTokens::RepeatedGroup(rep_group) => {
                rep_group.validate(cur_mode)
            },
            SeqTokens::AlteredIdent(alt_ident) => {
                alt_ident.validate(cur_mode)
            },
            SeqTokens::TTMatIden(mat_id) => {
                mat_id.validate(cur_mode)
            },
        }
    }
}
