use syn::{Result, ItemEnum, Error};
use proc_macro2::{TokenStream, Span};
use quote::quote;


// TODO what if single variant, check path in impl for hygiene, check variants are fieldless.
pub fn bitfield_specifier_impl(input : ItemEnum) -> Result<TokenStream> {

    // get enum id
    let enum_ident = input.ident;

    // num of variants
    let len = input.variants.len() as u128;

    if len == 0 {
        // no supported empty enums
        let err = Error::new(enum_ident.span(), "Empty Enums are not currently supported!");
        return Err(err);
    }

    if len == 1 {
        // no supported singleton enums
        let err = Error::new(enum_ident.span(), "Single variant Enums are not currently supported!");
        return Err(err);
    }

    if len.next_power_of_two() != len  {
        // enums must have power of 2 fields
        let err = Error::new(Span::call_site(), "BitfieldSpecifier expected a number of variants which is a power of 2");
        return Err(err);
    }

    // num of bits
    let bit_num = get_bits_needed(len);

    let match_variants = input.variants.into_iter().map(|variant| {
        let variant_id = variant.ident;

        quote!(
            x if x == (#enum_ident :: #variant_id as u64) => { #enum_ident :: #variant_id }
        )
    });

    let impl_specifier = quote!(
        impl bitfield::Specifier for #enum_ident {
            const BITS : usize = #bit_num as usize;
            type InOutType = Self ;


            //TODO make safer to stop anyone from using it
            fn from_u64(inp : u64) -> Self::InOutType {
                match inp {
                    #(#match_variants )*
                    _ => unreachable!()
                }

            }
    
            fn to_u64(inp : Self::InOutType) -> u64{
                inp as u64
            }
        }
    );

    //println!("{}", impl_specifier);


    Ok(impl_specifier)
}

fn get_bits_needed(num : u128) -> u128 {

    let next_pow_of_two = num.next_power_of_two();

    let mut pow: u128 = 0;

    loop {
        let pow_of_two = 2u128.pow(pow as u32);

        if pow_of_two == next_pow_of_two { return pow;}

        pow += 1;
    }

}

/*
fn main() {
    let x = 0;
    match x {
        te(Test::A) => (),
        _ => unreachable!()
    }

}

fn te(a : Test) -> u64 {
    a as u64
}


enum Test {
    A,
    B,
    C,
}*/