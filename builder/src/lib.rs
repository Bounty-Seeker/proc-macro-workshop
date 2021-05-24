use proc_macro::{TokenStream};
use proc_macro2::TokenStream as TS2;
use syn::{Data, DeriveInput, Fields, parse_macro_input, Type, parse2}; //, TypePath, Path, PathSegment};
use quote::{quote, format_ident};


#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // TokenStream::new()

    // get ident of given type
    let identi = input.ident;

    let fields;
    // get fields of struct
    if let Data::Struct(struct_data) = input.data {
        if let Fields::Named(field_data) = struct_data.fields {
            fields = field_data.named.into_iter();
        }
        // TODO what if unnamed field
        else { panic!() }
    }
    else { panic!() }
    // TODO what if not a struct

/*    fn alter_type(typ : Type) -> Type {
        let path: Punctuated<PathSegment, Colon2> = 
        Type::Path(
            TypePath {
                qself: None,
                path: Path {
                    leading_colon : None,
                    segments: [
                        PathSegment {
                            ident: "Option",
                            arguments: PathArguments::AngleBracketed(
                                AngleBracketedGenericArguments {
                                    args: [
                                        GenericArgument::Type(
                                            ...
                                        ),
                                    ],
                                },
                            ),
                        },
                    ],
                },
            },
        )
    }
*/
    fn optionise(typ : Type) -> Type {
        let typ : Type = parse2(quote!(Option<#typ>)).unwrap();
        typ
    }

    let field_names = fields.clone().map(|field| field.ident.unwrap());
    let types_names = fields.clone().map(|field| field.ty);
    let altered_type_names = fields.map(|field| optionise(field.ty));

    // create ident of our builder
    let builder_ident = format_ident!("{}Builder", identi);

    //let output_example = quote!(struct aaaa { #(#field_names : #types_names,)*};);

    let field_names1 = field_names.clone();
    let altered_type_names1 = altered_type_names;

    // create the builder struct and functions
    let output_builder_struct = quote!(
        pub struct #builder_ident {
            #( #field_names1 : #altered_type_names1 ),*
        }
    );

    let field_names2 = field_names.clone();

    let output_struct_methods = quote!(
        impl #identi {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #( #field_names2 : None),*
                }
            }
        }
    );

    let field_names3 = field_names;
    let type_names3 = types_names;

    let output_builder_struct_method = quote!(
        impl #builder_ident {

            #(fn #field_names3(&mut self, #field_names3: #type_names3) -> &mut Self {
                self.#field_names3 = Some(#field_names3);
                self
            })*
        }
        );

        let mut output = TS2::new();
        output.extend(output_struct_methods);
        output.extend(output_builder_struct);
        output.extend(output_builder_struct_method);
        output.into()
}