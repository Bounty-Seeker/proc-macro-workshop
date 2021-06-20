use super::bitfield_impl::checks::generate_multiple_of_eight_bits_check;
use super::bitfield_impl::checks::generate_variant_in_range_check;
use syn ::{Result};
use quote::{quote};
use proc_macro2::TokenStream;

pub fn create_checks_tokenstream() -> Result<TokenStream> {

    // generates required types for multiple of eight check in our library
    let multi_of_eight = generate_multiple_of_eight_bits_check()?;

    // generates required types for variant in range check in our library
    let vari_in_range = generate_variant_in_range_check()?;

    let output = quote!(
        pub mod checks {

            #multi_of_eight

            #vari_in_range
        }
    );

    Ok(output)

}
