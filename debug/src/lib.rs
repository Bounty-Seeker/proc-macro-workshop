use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use syn::{Attribute, Data, DeriveInput, Fields, LitStr, parse_macro_input, TraitBound,
            TypeParamBound, TypeParam, Type};
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
        // TODO Fix assumption
        let param_pthan_only = has_param_pthan_field(&param, &fields);
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
/// Only checks if their exists PhantomData<T> as only need such a type when no other type containing T exists.
fn has_param_pthan_field<'a>(param : &TypeParam, fields : impl IntoIterator<Item=&'a syn::Field>) -> bool {
    let phantom_type_tokens: proc_macro::TokenStream = quote!(PhantomData<#param>).into();
    let phantom_type = parse_macro_input::parse::<Type>(phantom_type_tokens).unwrap();
    for field in fields {
        let field_type = &field.ty;
        if *field_type == phantom_type { return true }
    }
    return false
}





