pub use multiple_of_eight_check::{generate_multiple_of_eight_bits_check, generate_multiple_of_eight_run_check};
pub use variant_in_range_check::{generate_variant_in_range_check, generate_variant_in_range_run_check};
pub use check_bits_attribute::generate_check_bits_attri_run_check;

mod multiple_of_eight_check;

mod variant_in_range_check;

mod check_bits_attribute {
    use proc_macro2::Span;
    use proc_macro2::TokenStream;
    use quote::quote_spanned;
    use syn::ItemStruct;
    use syn::Attribute;
    use syn::Result;
    use syn::{Meta,Lit};


    pub fn generate_check_bits_attri_run_check(input_struct : &ItemStruct) -> Result<TokenStream> {

        let mut output = TokenStream::new();

        for field in &input_struct.fields {

            // check if field has attribute
            let given_bits_num = check_attribute(&field.attrs)?;

            // if none then no bits attr so continue to next field
            if given_bits_num.is_none() {
                continue;
            }

            let (given_bits_num, attr_bits_span) = given_bits_num.unwrap();

            let field_ty = &field.ty;

            let field_bit_check = quote_spanned! {attr_bits_span =>
                let _ : [(); #given_bits_num as usize] = [();< #field_ty as Specifier>::BITS];
            };

            output.extend(field_bit_check);

        }

        Ok(output)
    }

    /// Goes through all attributes and returns Some((bits_len, span of num in attr)) if #[bits = _]
    fn check_attribute(attr_list : &[Attribute]) -> Result<Option<(u16, Span)>> {

        for attr in attr_list {

            let meta = attr.parse_meta()?;

            if let Meta::NameValue(inner) = meta {

                let path_ident = inner.path.get_ident();

                if path_ident.is_none() {
                    continue;
                }

                let path_ident = path_ident.unwrap();

                if *path_ident == "bits" {

                    if let Lit::Int(lit_int) = inner.lit {

                        let int_span = lit_int.span();

                        let int : u16 = lit_int.base10_parse()?;

                        return Ok( Some((int, int_span)));
                    }
                }

            }

        }

        Ok(None)
    }
}