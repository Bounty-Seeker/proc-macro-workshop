use proc_macro::{TokenStream};
use proc_macro2::TokenStream as TS2;
use syn::{Data, DeriveInput, Fields, parse_macro_input, Type, parse2}; //, TypePath, Path, PathSegment};
use quote::{quote, format_ident};


#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {

    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // get ident of given type
    let identi = input.ident;

    // get fields of struct
    let fields;
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

    /// Takes a syn::Type and returns a syn::Typ which is the original type wrapped in a option
    fn optionise(typ : Type) -> Type {
        let typ : Type = parse2(quote!(Option<#typ>)).unwrap();
        typ
    }

    // Iterator of the names of the field names of struct
    let field_names = fields.clone().map(|field| field.ident.unwrap());

    // Iterator of types of the fields of struct
    let types_names = fields.clone().map(|field| field.ty);

    // Iterator of types that have been optionised
    let altered_type_names = fields.map(|field| optionise(field.ty));

    // create ident of our builder
    let builder_ident = format_ident!("{}Builder", identi);


    // create the builder struct
    let field_names1 = field_names.clone();
    let altered_type_names1 = altered_type_names;

    let output_builder_struct = quote!(
        pub struct #builder_ident {
            #( #field_names1 : #altered_type_names1 ),*
        }
    );


    // create the builder function
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


    // creates set methods for builder structs
    let field_names3 = field_names.clone();
    let type_names3 = types_names;

    let output_builder_struct_method = quote!(
        impl #builder_ident {

            #(fn #field_names3(&mut self, #field_names3: #type_names3) -> &mut Self {
                self.#field_names3 = Some(#field_names3);
                self
            })*
        }
    );


    // creates build method
    let field_names4 = field_names.clone();
    let field_names_err_msg = field_names.clone()
                                .map(|iden| format!("{} field is not set!", iden));
    let field_names5 = field_names;

    let output_build_fn = quote!(
        impl #builder_ident {
            pub fn build(&mut self) -> Result<#identi, Box<dyn std::error::Error>> {

                #(
                    let #field_names4;
                    match self.#field_names4.take() {
                        Some(val) => { #field_names4 = val },
                        None => { return Err(#field_names_err_msg.into()); }
                    }
                )*

                Ok(#identi {
                    #(#field_names5),*
                })
            }
        }
    );


    // Put it all together
    let mut output = TS2::new();
    output.extend(output_struct_methods);
    output.extend(output_builder_struct);
    output.extend(output_builder_struct_method);
    output.extend(output_build_fn);
    output.into()
}