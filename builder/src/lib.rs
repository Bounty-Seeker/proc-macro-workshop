use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use syn::{Data, DeriveInput, Fields, parse_macro_input, Type, parse2, PathArguments, GenericArgument}; //, TypePath, Path, PathSegment};
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

    /// Takes a syn::Type and returns a (syn::Typ, bool)
    ///
    /// Returns the original type wrapped in a option and true bool value if original value is not already Option<T>.
    /// Returns the original type and false bool value if original value is already Option<T>
    fn optionise(typ : Type) -> (Type, bool) {

        let is_option = is_option(&typ);
        let out_typ : Type;
        if is_option {
            out_typ = typ;
        }
        else {
            out_typ = parse2(quote!(Option<#typ>)).unwrap();
        }
        (out_typ, !is_option)
    }

    /// Takes a syn::Type and returns a bool
    ///
    /// Returns true if input is Option<T>.
    /// Returns false otherwise.
    ///
    /// Compares it to:
    /// Type::Path(
    ///     TypePath {
    ///         qself: None,
    ///         path: Path {
    ///             segments: [
    ///                 PathSegment {
    ///                     ident: "Option",
    ///                     arguments: PathArguments::AngleBracketed(
    ///                         AngleBracketedGenericArguments {
    ///                             args: [
    ///                                 GenericArgument::Type(
    ///                                     ...
    ///                                 ),
    ///                             ],
    ///                         },
    ///                     ),
    ///                 },
    ///             ],
    ///         },
    ///     },
    /// )
    fn is_option(typ : &Type) -> bool {
        if let Type::Path(typ_path) = typ {
            let typ_path = &typ_path.path;
            let path_seg = &typ_path.segments;
            let intial_path_seg = path_seg.first().unwrap();
            let intial_path_seg_ident = &intial_path_seg.ident;
            return "Option" == intial_path_seg_ident.to_string()
        }
        false
    }

    /// Takes a syn::Type and returns a Option<syn::Type>
    ///
    /// Returns Some(T) if input is Option<T>.
    /// Returns None otherwise.
    ///
    fn get_option_generic(typ : &Type) -> Option<Type> {
        if let Type::Path(typ_path) = typ {
            let typ_path = &typ_path.path;
            let path_seg = &typ_path.segments;
            let intial_path_seg = path_seg.first().unwrap();
            let intial_path_seg_ident = &intial_path_seg.ident;
            if "Option" == intial_path_seg_ident.to_string() {
                let option_args = &intial_path_seg.arguments;
                if let PathArguments::AngleBracketed(option_generic_args) = option_args  {
                    let option_generic_argument = (option_generic_args.args).first().unwrap();
                    if let GenericArgument::Type(option_generic_type) = option_generic_argument {
                        return Some(option_generic_type.to_owned())
                    }
                }
            }
        }
        None
    }


    // Iterator of the names of the field names of struct
    let field_names = fields.clone().map(|field| field.ident.unwrap());

    // Iterator of types of the fields of struct
    let types_names = fields.clone().map(|field| field.ty);

    // Iterator of types that have been optionised with bool
    let altered_type_names_with_bool = fields.map(|field| optionise(field.ty));

    // Iterator of types that have been optionised
    let altered_type_names = altered_type_names_with_bool.clone().map(|(typ, _)| typ);

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

    let output_build_builder = quote!(
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
    let type_names3_removed_option = types_names.map(|typ| { if is_option(&typ) {get_option_generic(&typ).unwrap()} else {typ}});

    let output_builder_struct_method = quote!(
        impl #builder_ident {

            #(fn #field_names3(&mut self, #field_names3: #type_names3_removed_option) -> &mut Self {
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


    let get_fields = altered_type_names_with_bool.zip(field_names4).zip(field_names_err_msg)
                    .map(|(((_field_type, needs_set),field_ident), field_err_msg)| {
                        if needs_set {
                            quote!(
                                let #field_ident;
                                match self.#field_ident.take() {
                                    Some(val) => { #field_ident = val },
                                    None => { return Err(#field_err_msg.into()); }
                                }
                            )
                        }
                        else {
                            quote!(
                            let #field_ident = self.#field_ident.take();
                            )
                        }
                    });

    let output_build_fn = quote!(
        impl #builder_ident {
            pub fn build(&mut self) -> Result<#identi, Box<dyn std::error::Error>> {

                #(#get_fields)*

                Ok(#identi {
                    #(#field_names5),*
                })
            }
        }
    );


    // Put it all together
    let mut output = TS2::new();
    output.extend(output_build_builder);
    output.extend(output_builder_struct);
    output.extend(output_builder_struct_method);
    output.extend(output_build_fn);
    output.into()
}