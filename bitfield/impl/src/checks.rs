use super::bitfield_impl::checks::generate_multiple_of_eight_bits_check;
use syn ::Result;
use proc_macro2::TokenStream;

pub fn create_tokenstream() -> Result<TokenStream> {
    generate_multiple_of_eight_bits_check()
}
