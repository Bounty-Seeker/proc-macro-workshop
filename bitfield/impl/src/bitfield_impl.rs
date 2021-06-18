use super::ItemStruct;
use syn::{Result, Error};
use proc_macro2::{TokenStream, Span};
use quote::{quote, format_ident};

mod get_functions;

mod set_functions;

pub mod checks;

pub fn bitfield_impl(input_struct : ItemStruct) -> Result<TokenStream> {

    let struct_id = input_struct.ident;

    if let syn::Fields::Named(fields) = input_struct.fields {

        let fields_iter = fields.named.into_iter();

        let fields_type_iter = fields_iter.clone().map(|field| { field.ty} );

        //let fields_type_iter_clone = fields_type_iter.clone();

        //let const_size_id = format_ident!("{}_size", struct_id);

        let mut output = TokenStream::new();

        let fields_type_iter1 = fields_type_iter;

        let size = quote!((#(< #fields_type_iter1 as Specifier>::BITS + )* 0));

        let struct_ts = quote!(
            #[repr(C)]
            //#[derive(Debug)]
            pub struct #struct_id {
                data : [u8; #size /8],
            }
        );

        output.extend(struct_ts);

        let multiple_of_eight_check = checks::generate_multiple_of_eight_bits_check()?;

        output.extend(multiple_of_eight_check);

        let run_multiple_of_eight_check = checks::generate_multiple_of_eight_run_check(&size)?;


        //let fields_type_iter2 = fields_type_iter;

        let new_func = quote!(
            fn new() -> Self {
                #run_multiple_of_eight_check
                let data = [0; #size /8];
                #struct_id {
                    data
                }
            }
        );

        let mut set_funcs = TokenStream::new();

        let mut get_funcs = TokenStream::new();

        let mut start_bit = quote!(0);
        let mut end_bit = quote!(0);

        for field in fields_iter {

            let field_type = field.ty;

            end_bit.extend(quote!(+ < #field_type as Specifier>::BITS));

            let field_ident = field.ident.unwrap();

            // create get function for field
            let get_id = format_ident!("get_{}", field_ident);
            let get_func = get_functions::create_get_function(get_id, &start_bit, &end_bit, &field_type)?;
            get_funcs.extend(get_func);

            // create set function for field
            let set_id = format_ident!("set_{}", field_ident);
            let set_func = set_functions::create_set_function(set_id, &start_bit, &end_bit, &field_type)?;
            set_funcs.extend(set_func);

            start_bit = end_bit.clone();
        }

        let impl_funcs = quote!(
            impl #struct_id {

                #new_func

                #set_funcs

                #get_funcs
            }
        );

        output.extend(impl_funcs);


        Ok(output)

    } else {
        // create Error as we don't have named fields
        let err = Error::new(Span::call_site(),"Fields should be named");
        Err(err)
    }
}

/*
        let run_check = quote! (
            let _ : () = ZeroModEight
        );

        unimplemented!()

    }
}
*/
//struct Hello1 {}


/*
fn create_get_function(start_pos : TokenStream, end_position: TokenStream, id : Ident) -> Result<TokenStream> {

}

fn create_set_function(start_pos : TokenStream, end_position: TokenStream, id : Ident) -> Result<TokenStream> {

}
*/


/*
const fn get_location(start:u128, end:u128) -> impl Iterator<Item = (usize, u8)> {
    todo!()
}*/
/*
struct Decoder {

}

struct Encoder {

}

fn main() {

let data = [1,2,3,45,6,8,7,66];



}

*/

































