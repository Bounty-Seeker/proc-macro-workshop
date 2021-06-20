use super::super::*;
use syn::ItemEnum;
use quote::{quote_spanned, spanned::Spanned};

pub fn generate_variant_in_range_check() -> Result<TokenStream> {

    let output = quote!(

        pub trait DiscriminantInRange {
            type Check;
        }

        pub struct True;

        impl DiscriminantInRange for True {
            type Check = ();
        }

        pub struct False;

        pub trait TransformToTypeBool {
            type TypeBool;
        }

        impl TransformToTypeBool for [();0] {
            type TypeBool = False;
        }

        impl TransformToTypeBool for [();1] {
            type TypeBool = True;
        }

        pub type CheckIfVariantInRange<T> = <<T as TransformToTypeBool>::TypeBool as DiscriminantInRange>::Check;

    );

    Ok(output)

}


pub fn generate_variant_in_range_run_check(inp:&ItemEnum, bit_num : u128) -> Result<TokenStream> {

    let mut output = TokenStream::new();

    let enum_id = &inp.ident;

    let bound = quote!(
        (2u128.pow(#bit_num as u32))
    );

    let variants = (&inp.variants).into_iter();

    for variant in variants {

        let var_span = variant.__span();

        let var_id = &variant.ident;

        let var_output = quote_spanned!(var_span =>
            let _ :  bitfield::checks::CheckIfVariantInRange<[(); if (#enum_id :: #var_id as u128) < #bound {1} else {0} ]>;
        );

        output.extend(var_output);

    }

    Ok(output)
}
