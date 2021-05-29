use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::quote;

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {

    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name_str = input.ident.to_string();
    let struct_name = input.ident;

    let fields = {
        match input.data {
            Data::Struct(struct_data) => {
                match struct_data {
                    syn::DataStruct{
                        struct_token: _,
                        fields: Fields::Named(fields),
                        semi_token: _
                    } => { fields.named }
                    _ => { panic!() }
                }
            }
            _ => { panic!() }
        }
    };

    let field_names = fields.into_iter().map(|field| field.ident.unwrap());

    let field_names_str = field_names.clone().map(|field_name| field_name.to_string());

    let mut output = TS2::new();

    let debug_impl = quote!(
        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(#struct_name_str)
                #(.field(#field_names_str , &self.#field_names))*
                .finish()
            }
        }
    );

    output.extend(debug_impl);

    output.into()
}












