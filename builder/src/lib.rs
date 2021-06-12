use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::{format_ident, quote};
use syn::{
    parse2, parse_macro_input, token::Eq, Data, DeriveInput, Error, Field, Fields, GenericArgument,
    Ident, Lit, LitStr, Meta, MetaNameValue, NestedMeta, Path, PathArguments, Type,
};

#[proc_macro_derive(Builder, attributes(builder))]
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
        else {
            panic!()
        }
    } else {
        panic!()
    }
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
    fn optionise(typ: Type) -> (Type, bool) {
        let is_option = is_option(&typ);
        let out_typ: Type;
        if is_option {
            out_typ = typ;
        } else {
            out_typ = parse2(quote!(std::option::Option<#typ>)).unwrap();
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
    fn is_option(typ: &Type) -> bool {
        if let Type::Path(typ_path) = typ {
            let typ_path = &typ_path.path;
            let path_seg = &typ_path.segments;
            let intial_path_seg = path_seg.first().unwrap();
            let intial_path_seg_ident = &intial_path_seg.ident;
            let intial_path_seg_args = &intial_path_seg.arguments;
            let to_match = Ident::new("Option", intial_path_seg_ident.span());
            let ident_match = &to_match == intial_path_seg_ident;
            if ident_match {
                return one_generic_type(intial_path_seg_args);
            }
        }
        false
    }

    /// Takes &PathArgument and returns bool
    /// If has only one generic type i.e <T> returns true.
    /// Returns False otherwise
    fn one_generic_type(path_args: &PathArguments) -> bool {
        if let PathArguments::AngleBracketed(gen_args) = path_args {
            let gen_args = &gen_args.args;
            if gen_args.len() == 1 {
                let gen_arg = gen_args.first().unwrap();
                if let GenericArgument::Type(_) = gen_arg {
                    return true;
                }
            }
        }

        false
    }

    /// Takes a syn::Type and returns a Option<syn::Type>
    ///
    /// Returns Some(T) if input is Option<T>.
    /// Returns None otherwise.
    ///
    fn get_option_generic(typ: &Type) -> Option<Type> {
        if let Type::Path(typ_path) = typ {
            let typ_path = &typ_path.path;
            let path_seg = &typ_path.segments;
            let intial_path_seg = path_seg.first().unwrap();
            let intial_path_seg_ident = &intial_path_seg.ident;
            let to_match = Ident::new("Option", intial_path_seg_ident.span());
            if &to_match == intial_path_seg_ident {
                let option_args = &intial_path_seg.arguments;
                if let PathArguments::AngleBracketed(option_generic_args) = option_args {
                    let option_generic_argument = (option_generic_args.args).first().unwrap();
                    if let GenericArgument::Type(option_generic_type) = option_generic_argument {
                        return Some(option_generic_type.to_owned());
                    }
                }
            }
        }
        None
    }

    /// Takes a syn::Type and returns a bool
    ///
    /// Returns true if input is Vec<T>.
    /// Returns false otherwise.
    fn is_vec(typ: &Type) -> bool {
        if let Type::Path(typ_path) = typ {
            let typ_path = &typ_path.path;
            let path_seg = &typ_path.segments;
            let intial_path_seg = path_seg.first().unwrap();
            let intial_path_seg_ident = &intial_path_seg.ident;
            let intial_path_seg_args = &intial_path_seg.arguments;
            let to_match = Ident::new("Vec", intial_path_seg_ident.span());
            let ident_match = &to_match == intial_path_seg_ident;
            if ident_match {
                return one_generic_type(intial_path_seg_args);
            }
        }
        false
    }

    /// Takes a syn::Type and returns a Option<syn::Type>
    ///
    /// Returns Some(T) if input is Vec<T>.
    /// Returns None otherwise.
    ///
    fn get_vec_generic(typ: &Type) -> Option<Type> {
        if let Type::Path(typ_path) = typ {
            let typ_path = &typ_path.path;
            let path_seg = &typ_path.segments;
            let intial_path_seg = path_seg.first().unwrap();
            let intial_path_seg_ident = &intial_path_seg.ident;
            let to_match = Ident::new("Vec", intial_path_seg_ident.span());
            if &to_match == intial_path_seg_ident {
                let option_args = &intial_path_seg.arguments;
                if let PathArguments::AngleBracketed(option_generic_args) = option_args {
                    let option_generic_argument = (option_generic_args.args).first().unwrap();
                    if let GenericArgument::Type(option_generic_type) = option_generic_argument {
                        return Some(option_generic_type.to_owned());
                    }
                }
            }
        }
        None
    }

    /// Takes a &Field and returns bool.
    /// Returns true if attributes contain a correct builder each attribute.
    /// Returns false otherwise
    fn has_correct_builder_attribute(fiel: &Field) -> bool {
        let attr_values = (&fiel.attrs).iter().filter_map(|attr| {
            if let Some(nested) = is_builder(&attr.parse_meta().unwrap()) {
                return get_nested(nested);
            }
            None
        });
        attr_values.clone().count() >= 1
    }

    // Iterator of the names of the field names of struct
    let field_names = fields.clone().map(|field| field.ident.unwrap());

    // Iterator of types of the fields of struct
    let types_names = fields.clone().map(|field| field.ty);

    // Iterator of types that have been optionised with bool
    let altered_type_names_with_bool = fields.clone().map(|field| optionise(field.ty));

    // Iterator of types that have been optionised
    let _altered_type_names = altered_type_names_with_bool.map(|(typ, _)| typ);

    // create ident of our builder
    let builder_ident = format_ident!("{}Builder", identi);

    // create the builder struct
    let field1 = fields.clone();
    //let altered_type_names1 = altered_type_names;
    let struct_fields = field1.into_iter().map(|fiel| {
        let builder_field_type_name;
        let field_name = fiel.ident.clone().unwrap();
        if has_correct_builder_attribute(&fiel) {
            builder_field_type_name = fiel.ty;
        } else {
            builder_field_type_name = optionise(fiel.ty).0;
        }
        quote!(#field_name : #builder_field_type_name)
    });

    let output_builder_struct = quote!(
        pub struct #builder_ident {
            #( #struct_fields ),*
        }
    );

    // create the builder function
    let field2 = fields.clone();
    let set_builder_fields = field2.into_iter().map(|fiel| {
        let field_name = fiel.ident.clone().unwrap();
        if has_correct_builder_attribute(&fiel) {
            quote!(#field_name : Vec::new())
        } else {
            quote!(#field_name : None)
        }
    });

    let output_build_builder = quote!(
        impl #identi {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #( #set_builder_fields),*
                }
            }
        }
    );

    // creates set methods for builder structs
    let _field_names3 = field_names.clone();

    let field3 = fields.clone();
    let _type_names3_removed_option = types_names.map(|typ| {
        if is_option(&typ) {
            get_option_generic(&typ).unwrap()
        } else {
            typ
        }
    });

    let fields_setters = field3.into_iter().map(|fiel| {
        let mut attr_values = fiel.attrs.iter().filter_map(|attr| {
            let meta = attr.parse_meta().unwrap();
            let nested = is_builder(&meta)?;
            let attr_span = attr.bracket_token.span;
            Some((nested.clone(), attr_span))
        });

        //if attr_values.clone().count() > 1 { panic!() };
        let attr_value;
        //check spelled correctly
        if let Some((nested, attr_span)) = attr_values.next() {
            if let Some(str_lit) = get_nested(&nested) {
                attr_value = Some(str_lit);
            } else {
                //error
                let err = Error::new(attr_span, "expected `builder(each = \"...\")`");
                return err.to_compile_error();
            }
        } else {
            attr_value = None;
        }

        // check if more than one builder attribute
        if attr_values.next().is_some() {
            panic!()
        }

        // for attr in attributes {
        //     println!("Attr: Path: {:?}", /*attr.path.segments.into_iter().next());
        //     println!("Tokens: {:?} \n \n", attr.parse_meta().unwrap());
        //     if let Some(nested) = is_builder(&attr.parse_meta().unwrap()) {
        //         println!("Is builder \n Nested : {:?} \n", nested);
        //         println!("extracted value : {:?} \n \n \n", get_nested(nested));
        //     };
        //     println!("Tokens: {:?}", attr.parse_meta().unwrap());
        // }
        //let attr_value = attr_values.next();
        let field_name = fiel.ident.unwrap();
        let type_name_removed_option = {
            let typ = fiel.ty.clone();
            if is_option(&typ) {
                get_option_generic(&typ).unwrap()
            } else {
                typ
            }
        };

        if let Some(attr_str) = attr_value {
            let mut setters = TS2::new();

            let attr_ident: Ident = attr_str.parse().unwrap();

            let type_name_remove_vec = {
                let typ = fiel.ty;
                if is_vec(&typ) {
                    get_vec_generic(&typ).unwrap()
                } else {
                    panic!()
                }
            };

            // generate function for each attribute
            let set_each = quote!(
                fn #attr_ident(&mut self, #field_name : #type_name_remove_vec) -> &mut Self {
                    self.#field_name.push(#field_name);
                    self
                }
            );

            setters.extend(set_each);
            if attr_ident != field_name {
                let set_total = quote!(
                    fn #field_name(&mut self, #field_name: #type_name_removed_option) -> &mut Self {
                        self.#field_name = #field_name;
                        self
                    }
                );
                setters.extend(set_total);
            }
            setters
        } else {
            quote!(
                fn #field_name(&mut self, #field_name: #type_name_removed_option) -> &mut Self {
                    self.#field_name = Some(#field_name);
                    self
                }
            )
        }
    });

    let output_builder_struct_method = quote!(
        impl #builder_ident {
            #(#fields_setters)*
        }
    );

    /// Takes a &syn::Meta and returns a Option<&syn::NestedMeta>
    /// Returns None if path does not match expected builder macro or if nested has more than one part.
    /// Returns Some(&Meta) if matching and &NestedMeta points to the nested part which also has only one element.
    ///
    ///    List(
    ///        MetaList {
    ///            path: Path {
    ///                leading_colon: None,
    ///                segments: [
    ///                    PathSegment {
    ///                        ident: Ident {
    ///                            ident: "builder",
    ///                            span: #0 bytes(298..305)
    ///                        },
    ///                        arguments: None
    ///                    }
    ///                    ]
    ///                },
    ///            paren_token: Paren,
    ///            nested: [
    ///                Meta(
    ///                    NameValue(
    ///                        MetaNameValue {
    ///                            path: Path {
    ///                                leading_colon: None,
    ///                                segments: [
    ///                                    PathSegment {
    ///                                        ident: Ident {
    ///                                            ident: "each",
    ///                                            span: #0 bytes(306..310)
    ///                                        },
    ///                                        arguments: None
    ///                                        }
    ///                                    ]
    ///                                },
    ///                            eq_token: Eq,
    ///                            lit: Str(
    ///                                LitStr {
    ///                                    token: "env"
    ///                                    }
    ///                                )
    ///                        }
    ///                    )
    ///                )
    ///            ]
    ///        }
    ///    )
    fn is_builder(inp: &Meta) -> Option<&NestedMeta> {
        if let Meta::List(met_list) = inp {
            let inp_path = &met_list.path;
            let inp_nested = &met_list.nested;
            let path_seg = &inp_path.segments;
            if path_seg.len() != 1 {
                return None;
            };
            let intial_path_seg = path_seg.first().unwrap();
            let intial_path_seg_ident = &intial_path_seg.ident;
            let to_match = Ident::new("builder", intial_path_seg_ident.span());
            if to_match != *intial_path_seg_ident {
                return None;
            }
            if inp_nested.len() != 1 {
                return None;
            }
            return inp_nested.first();
        }
        None
    }

    /// Takes &NestedMeta and returns Option<syn::LitStr>
    /// returns None if nested doesn't match 'each = "Foo"' form
    /// returns Some(LitStr(_)) for the Foo str.
    fn get_nested(nested: &NestedMeta) -> Option<syn::LitStr> {
        match nested {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path:
                    Path {
                        leading_colon: None,
                        segments: each_segment,
                    },
                eq_token: Eq { .. },
                lit: Lit::Str(variable_str_lit @ LitStr { .. }),
            })) => {
                if each_segment.len() != 1 {
                    return None;
                }
                let each_path_segment = each_segment.first().unwrap();
                if each_path_segment.arguments != PathArguments::None {
                    return None;
                }
                let each_ident = &each_path_segment.ident;
                let to_match = Ident::new("each", each_ident.span());
                if to_match == *each_ident {
                    return Some(variable_str_lit.clone());
                }
                None
            }
            _ => None,
        }
    }

    /*
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
    );*/

    // creates build method
    let _field_names4 = field_names.clone();
    let _field_names_err_msg = field_names
        .clone()
        .map(|iden| format!("{} field is not set!", iden));
    let field_names5 = field_names;

    // create the builder function
    let field4 = fields;
    let get_fields = field4.into_iter().map(|fiel| {
        let has_attribute = has_correct_builder_attribute(&fiel);
        let field_name = fiel.ident.clone().unwrap();
        let field_err_msg = format!("{} field is not set", fiel.ident.clone().unwrap());
        let need_set = optionise(fiel.ty).1;
        if has_attribute {
            quote!(
            let #field_name = std::mem::take(&mut self.#field_name);
            )
        } else if need_set {
            quote!(
                let #field_name;
                match self.#field_name.take() {
                    Some(val) => { #field_name = val },
                    None => { return Err(#field_err_msg.into()); }
                }
            )
        } else {
            quote!(
            let #field_name = self.#field_name.take();
            )
        }
    });
    /*    let get_fields = altered_type_names_with_bool.zip(field_names4).zip(field_names_err_msg)
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
    });*/

    let output_build_fn = quote!(
        impl #builder_ident {
            pub fn build(&mut self) -> std::result::Result<#identi, std::boxed::Box<dyn std::error::Error>> {

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
