use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use syn::{Attribute, Data, DeriveInput, Fields, LitStr, parse_macro_input, TraitBound,
            TypeParamBound, TypeParam, Type, PathArguments, GenericArgument};
use quote::quote;

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {

    // Parse the input tokens into a syntax tree
    let mut input = parse_macro_input!(input as DeriveInput);


    let struct_name_str = input.ident.to_string();
    let struct_name = input.ident;

    let fields = {
        match input.data {
            Data::Struct(
                syn::DataStruct{
                    struct_token: _,
                    fields: Fields::Named(fields),
                    semi_token: _
                }
            ) => { fields.named }
            _ => { panic!() }
        }
    };

    //println!("{:?}", impl_generics);

    let trait_bound = quote!(std::fmt::Debug).into();
    let trait_bound = parse_macro_input!(trait_bound as TraitBound);
    //println!("Trait Bound {:?}",&trait_bound);
    let trait_bound: TypeParamBound = trait_bound.into();

    input.generics.type_params_mut().for_each(|param| {
        //println!("input {:?}", param);
        // Assuming that phantomData<T> only appears when we don't have a T type otherwise
        let param_pthan_only = only_param_pthan_field(&param, &fields);
        if !param_pthan_only {
            param.bounds.extend(std::iter::once(trait_bound.clone()));
        }
    });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();


    let field_names = fields.clone().into_iter().map(|field| field.ident.unwrap());

    let field_names_str = field_names.clone().map(|field_name| field_name.to_string());

    let field_attrs = fields.into_iter().map(|field| field.attrs)
                                                    .map(|attr_vec| {
                                                        let mut attr_iter= attr_vec.into_iter()
                                                                .filter_map(|attr| get_attr_value(&attr));
                                                        if let Some(attr_value) = attr_iter.next() {
                                                            attr_value
                                                        }
                                                        else {"{:?}".to_string()}
                                                    });


    let mut output = TS2::new();

    let debug_impl = quote!(
        impl #impl_generics std::fmt::Debug for #struct_name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(#struct_name_str)
                #(.field(#field_names_str , &std::format_args!(#field_attrs, &self.#field_names)))*
                .finish()
            }
        }
    );

    output.extend(debug_impl);

    output.into()
}

/// Checks attributes are debug and of the correct form then returns the String.
fn get_attr_value(attr : &Attribute) -> Option<String> {
    let attr = attr.parse_meta().unwrap();
    match attr {
        syn::Meta::NameValue(
            syn::MetaNameValue {
                path: syn::Path {
                    leading_colon: None,
                    segments: path_seg ,
                },
                eq_token: syn::token::Eq{..},
                lit: syn::Lit::Str( lit_str @ LitStr { .. })
            }
        ) => {
            if path_seg.len() != 1 { return None }
            let path_seg = path_seg.first().unwrap();
            match path_seg {
                syn::PathSegment{
                    ident: debug_ident,
                    arguments: syn::PathArguments::None
                } => {
                    let to_match = syn::Ident::new("debug", debug_ident.span());
                    if to_match == *debug_ident { return Some(lit_str.value())}
                    None
                },
                _ => None,
            }
        }
        _ => None
    }
}

/// Checks if a type parameter only appears in PhantomData<T>
fn only_param_pthan_field<'a>(param : &TypeParam, fields : impl IntoIterator<Item=&'a syn::Field>) -> bool {
    let phantom_type_tokens: proc_macro::TokenStream = quote!(PhantomData<#param>).into();
    let phantom_type = parse_macro_input::parse::<Type>(phantom_type_tokens).unwrap();

    let param_ident = &param.ident;
    let param_tokens: proc_macro::TokenStream = quote!(#param_ident).into();
    let param_type: Type = parse_macro_input::parse::<Type>(param_tokens).unwrap();
    println!("{:?}", param_type);
    for field in fields {
        let field_type = &field.ty;
        if *field_type == phantom_type { continue; }
        if contains_param_type(&param_type, field_type) { return false; }
    }
    true
}

//TODO improve and check logic
fn contains_param_type(param_type: &Type, to_check_against: &Type) -> bool {
    if param_type == to_check_against { return true; }
    match to_check_against {
        Type::Array(type_array) => { contains_param_type(param_type,&type_array.elem) }
        Type::BareFn(type_bare_fn) => {
            let function_args = &type_bare_fn.inputs;
            for function_arg in function_args {
                if contains_param_type(param_type, &function_arg.ty) { return true }
            }

            if let syn::ReturnType::Type(_, return_type) = &type_bare_fn.output {
                return contains_param_type(param_type,&return_type)
            }
            false
        }
        Type::Group(type_group) => { contains_param_type(param_type,&type_group.elem) }
        Type::ImplTrait(_type_impl_trait) => {false}
        Type::Infer(_type_infer) => { panic!("Can't tell _") }
        Type::Macro(_type_macro) => { panic!("Can't tell as macro") }
        Type::Never(_type_never) => { false}
        Type::Paren(type_paren) => { contains_param_type(param_type,&type_paren.elem) }
        Type::Path(type_path) => {
            for segments in &type_path.path.segments {

                match &segments.arguments {
                    PathArguments::None => continue,
                    PathArguments::AngleBracketed(args) => {
                        let args = &args.args;
                        for arg in args {
                            match arg {
                            GenericArgument::Lifetime(_lifetime) => continue,
                            GenericArgument::Type(typ) => {
                                if contains_param_type(param_type, &typ) { return true; }
                            }
                            GenericArgument::Binding(binding) => {
                                if contains_param_type(param_type, &binding.ty) { return true; }
                            }
                            GenericArgument::Constraint(_constraint) => continue,
                            GenericArgument::Const(_expr) => { panic!("Can't tell as expr") }
                            }
                        }
                    }
                    PathArguments::Parenthesized(args) => {
                        let function_args = &args.inputs;
                        for function_arg in function_args {
                            if contains_param_type(param_type, &function_arg) { return true }
                        }

                        if let syn::ReturnType::Type(_, return_type) = &args.output {
                            return contains_param_type(param_type,&return_type)
                        }
                        return false
                    }
                }
            }

            false
        }
        Type::Ptr(type_ptr) => { contains_param_type(param_type,&type_ptr.elem) }
        Type::Reference(type_reference) => { contains_param_type(param_type,&type_reference.elem)}
        Type::Slice(type_slice) => { contains_param_type(param_type,&type_slice.elem) }
        Type::TraitObject(_type_trait_object) => { false }
        Type::Tuple(type_tuple) => {
            for tuple_term_type in &type_tuple.elems {
                if contains_param_type(param_type,&tuple_term_type) { return true; }
            }
            false
        }
        Type::Verbatim(_token_stream) => { panic!("Can't tell as TokenStream") }
        _ => panic!(),
    }
}



