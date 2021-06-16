use proc_macro::TokenStream;
use syn::{parse::Nothing, parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {

    // check no args
    parse_macro_input!(args as Nothing);

    // parse input as ItemStruct
    let input_struct = parse_macro_input!(input as ItemStruct);

    // apply the macro
    let output = bitfield_impl::bitfield_impl(input_struct);

    // if output is error covert to compiler error and return
    output.unwrap_or_else(|err| err.to_compile_error()).into()
}


mod bitfield_impl {
    use super::ItemStruct;
    use syn::{Result, Error};
    use proc_macro2::{TokenStream, Span};
    use quote::{ quote};

    pub fn bitfield_impl(input_struct : ItemStruct) -> Result<TokenStream> {

        let struct_id = input_struct.ident;

        if let syn::Fields::Named(fields) = input_struct.fields {

            let fields_iter = fields.named.into_iter();

            let fields_type_iter = fields_iter.map(|field| { field.ty} );

            //let fields_type_iter_clone = fields_type_iter.clone();

            //let const_size_id = format_ident!("{}_size", struct_id);

            let mut output = TokenStream::new();

            //println!("here");

            //let const_ts = quote!(const #const_size_id : usize = #(< #fields_type_iter as Specifier>::BITS + )* 0;);

            //println!("test: {}", test_ts);

            //output.extend(const_ts);

            let t_s = quote!(
                #[repr(C)]
                pub struct #struct_id {
                    data : [u8; (#(< #fields_type_iter as Specifier>::BITS + )* 0)/8],
                }
            );

            output.extend(t_s);

            Ok(output)

        } else {
            // create Error as we don't have named fields
            let err = Error::new(Span::call_site(),"Fields should be named");
            Err(err)
        }
    }
}