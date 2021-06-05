use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TS2, Span};
use syn::{Attribute, Data, DeriveInput, Error, Fields, GenericArgument, Ident, LitStr, Meta, MetaList, MetaNameValue, NestedMeta, Path, PathArguments, PathSegment, ReturnType, TraitBound, Type, TypeParam, TypeParamBound, TypePath, WherePredicate, parse_macro_input, punctuated::Punctuated, token::{Comma, Colon2}};
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

    let mut associated_types_vec = Vec::new();

    let mut attr_bounds = Vec::new();

    for attr in &input.attrs {
        let attr_bound= try_attribute(attr);

        if let Some(attr_bound) = attr_bound {
            attr_bounds.push(attr_bound);
        }
    }

    input.generics.type_params_mut().for_each(|param| {
        //println!("input {:?}", param);
        // Assuming that phantomData<T> only appears when we don't have a T type otherwise
        let param_pthan_only = only_param_pthan_field(&param, &fields);
        //println!("Only phantom {:?} for {:?}", param_pthan_only, param.ident);
        if !param_pthan_only {
            //param.bounds.extend(std::iter::once(trait_bound.clone()));

            //println!("Hi");

            //TODO move to own function
            for field in &fields {

                //println!("Hello");
                let assoc_types = find_assoc_types(&param.ident, &field.ty);

                associated_types_vec.extend(assoc_types);
            }
        }
    });

    let assoc_where_pred = associated_types_vec.into_iter().map(|typ| {

        let mut type_bound = Punctuated::new();
        type_bound.extend(std::iter::once(trait_bound.clone()));
        let assoc_pred_typ = syn::PredicateType{
            lifetimes : None,
            bounded_ty: typ,
            colon_token: syn::token::Colon{spans:[Span::call_site()]},
            bounds : type_bound
        };
        WherePredicate::Type(assoc_pred_typ)
    });
    //println!("jfgjf");
    let where_clause = input.generics.make_where_clause();

    let mut predicates: Punctuated<WherePredicate, Comma> = Punctuated::new();

    if attr_bounds.is_empty() {
        predicates.extend(assoc_where_pred);

    } else {
        predicates.extend(attr_bounds.into_iter());
    }
    //println!("predicates {:?}", predicates);
    where_clause.predicates = predicates;
    //println!("where : {:?}", where_clause);

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();


    let field_names = fields.clone().into_iter().map(|field| {/*println!("ident");*/ field.ident.unwrap()});

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
    let param_ident = &param.ident;
    //println!("param {:?} \n end", param);
    //let phantom_type_tokens: proc_macro::TokenStream = quote!(PhantomData<#param_ident>).into();
    //println!("phan type {:?}", phantom_type_tokens);
    //let _phantom_type = parse_macro_input::parse::<Type>(phantom_type_tokens).unwrap();


    let param_tokens: proc_macro::TokenStream = quote!(#param_ident).into();
    //println!("param type");
    let _param_type: Type = parse_macro_input::parse::<Type>(param_tokens).unwrap();
    //println!("param_type {:?} \n end", param_type);
    for field in fields {
        let field_type = &field.ty;


        //Does field type contain param
        let field_contains_param = contains_param_type(&param_ident, field_type);

        //println!("Field {:?} \n contains param {:?} \n is {}\n ",field_type,param_type, field_contains_param);
        if !field_contains_param {continue;}

        // From here Know the field must contain param

        //println!("field_type {:?} \n end ", field_type);
        if let Type::Path(type_path) = &field_type {
            //Possibly PhantomData<T>
            let path = &type_path.path;
            let type_ident = &path.segments.first().unwrap().ident;

            let phantom_ident = Ident::new("PhantomData", Span::clone(&type_ident.span()));

            if phantom_ident == *type_ident { continue; }
            else { return false; }

                //let phantom_match = phantom_ident == segments.ident;
            

        } else { // Not PhantomData<T> case so if contains T then return false
            return false;
        }
        //if *field_type == phantom_type { continue; }
    }
    true
}


/// finds generic associated types
fn find_assoc_types(param_ident : &Ident, field_type : &Type) -> Vec<Type> {

    let mut output_vec = Vec::new();

    // TODO type traits and impl traits and Qself stuff get to work
    match field_type {
        Type::Array(type_array) => { find_assoc_types(param_ident, &*type_array.elem) }
        Type::BareFn(type_bare_fn) => {
            let function_args = &type_bare_fn.inputs;
            for function_arg in function_args {
                let input_arg_assoc_types = find_assoc_types(param_ident, &function_arg.ty);
                output_vec.extend(input_arg_assoc_types);
            }

            if let syn::ReturnType::Type(_, return_type) = &type_bare_fn.output {
                let output_arg_assoc_types = find_assoc_types(param_ident, &*return_type);
                output_vec.extend(output_arg_assoc_types);
            }
            output_vec
        }
        Type::Group(type_group) => { find_assoc_types(param_ident, &*type_group.elem) }
        Type::ImplTrait(_type_impl_trait) => { panic!("Can't handle impl traits currently") }
        Type::Infer(_type_infer) => { output_vec }
        Type::Macro(_type_macro) => { output_vec }
        Type::Never(_type_never) => { output_vec }
        Type::Paren(type_paren) => { find_assoc_types(param_ident, &*type_paren.elem) }
        Type::Path(type_path) => {

            let mut path_segments_iter = type_path.path.segments.iter();

            while let Some(segment) = path_segments_iter.next() {

                let is_ident = segment.ident == *param_ident;


                if is_ident {
                    let mut associ_path_segs: Punctuated<PathSegment,Colon2> = Punctuated::<PathSegment,Colon2>::new();
                    associ_path_segs.extend(std::iter::once(segment.clone()));
                    associ_path_segs.extend(path_segments_iter.cloned());
                    
                    //println!("\n \n \n associated path: {:?} \n\n\n", associ_path_segs);
                    let assoc_path : Path = Path{
                        leading_colon:None,
                        segments:associ_path_segs,
                    };
                    let type_path = TypePath {
                        qself:None,
                        path: assoc_path,
                    };
                    let ty:Type = type_path.into();
                    output_vec.extend(std::iter::once(ty));
                    return output_vec;
                }

                let segment_args = &segment.arguments;

                match segment_args {
                    PathArguments::None => {}
                    PathArguments::AngleBracketed(angle_bracket) => {
                        for argument in &angle_bracket.args {
                            if let GenericArgument::Type(ty) = argument {
                                let generic_assoc_types = find_assoc_types(param_ident, ty);
                                output_vec.extend(generic_assoc_types);
                            }
                        }
                    }
                    PathArguments::Parenthesized(parenthesize) => {
                        for ty in &parenthesize.inputs {
                            let generic_assoc_types = find_assoc_types(param_ident, ty);
                            output_vec.extend(generic_assoc_types);
                        }

                        if let ReturnType::Type(_, ty) = &parenthesize.output {
                            let generic_assoc_types = find_assoc_types(param_ident, ty);
                            output_vec.extend(generic_assoc_types);
                        }
                    }
                }

            }

            output_vec
        }
        Type::Ptr(type_ptr) => { find_assoc_types(param_ident, &*type_ptr.elem) }
        Type::Reference(type_reference) => { find_assoc_types(param_ident, &*type_reference.elem) }
        Type::Slice(type_slice) => { find_assoc_types(param_ident, &*type_slice.elem) }
        Type::TraitObject(_type_trait_object) => { panic!("Can't handle dynamic traits currently")  }
        Type::Tuple(type_tuple) => {
            for tuple_field in &type_tuple.elems{
                let tuple_assoc_types = find_assoc_types(param_ident, tuple_field);
                output_vec.extend(tuple_assoc_types);
            }
            output_vec
        }
        Type::Verbatim(_token_stream) => { panic!("Can't find from token_stream") }
        _ => panic!()
    }

}

fn try_attribute(attr : &Attribute) -> Option<WherePredicate> {

    //println!("{:?}", attr.parse_meta());
    //let mut output = Vec::new();

    let meta = attr.parse_meta();
    let meta = match meta {
        Ok(meta) => meta,
        Err(_) => return None
    };

    //println!("{:?}", meta);

    let meta = match meta {
        Meta::List(meta) => meta,
        _ => return None
    };


    let MetaList {
        path,
        paren_token: _,
        nested,
    } = meta;

    if let syn::Path{
        leading_colon:None,
        segments
    } = path {
        if !(segments.len() == 1) { return None }

        let segment = segments.first().unwrap();

        if let PathSegment{
            ident,
            arguments: PathArguments::None,
        } = segment {
            let match_ident = Ident::new("debug", ident.span());
            if !(match_ident == *ident) {
                    return None
            }
            //let args = attr.parse_args();
            //println!()
        }

        //println!("Found debug \n\n\n");

        for nest in nested {

            if let NestedMeta::Meta(
                Meta::NameValue(name_value)
            ) = nest {

                let MetaNameValue{
                    path,
                    eq_token:_,
                    lit
                } = name_value;

                if let syn::Path{
                    leading_colon:None,
                    segments
                } = path {
                    if !(segments.len() == 1) { return None }
            
                    let segment = segments.first().unwrap();
            
                    if let PathSegment{
                        ident,
                        arguments: PathArguments::None,
                    } = segment {
                        let match_ident = Ident::new("bound", ident.span());
                        if !(match_ident == *ident) {
                                return None
                        }
                    }
                }

                //println!("Found bound \n\n\n");

                if let syn::Lit::Str(lit) = lit {
                    //println!("Found lit {:?} \n\n\n", &lit.value()[..]);

                    let parse_attempt: Result<WherePredicate,Error> = syn::parse_str(&lit.value()[..]);
                    //println!("{:?}", parse_attempt);
                    match parse_attempt {
                        Ok(trait_bound) => { //println!("Found parse \n\n\n");
                        return Some(trait_bound) }
                        _ => { //println!("Not parsed \n\n\n");
                        return None }
                    }
                } else { return None; }

            } else { continue; }

        }

    }

    None
}

/*
Path(
    TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [
                PathSegment {
                    ident: Ident {
                        ident: "Vec",
                        span: #0 bytes(300..303)
                    },
                    arguments: AngleBracketed(
                        AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Lt,
                            args: [
                                Type(
                                    Path(
                                        TypePath {
                                            qself: Some(
                                                QSelf {
                                                    lt_token: Lt,
                                                    ty: Path(
                                                        TypePath {
                                                            qself: None,
                                                            path: Path {
                                                                leading_colon: None,
                                                                segments: [
                                                                    PathSegment {
                                                                        ident: Ident {
                                                                            ident: "T",
                                                                            span: #0 bytes(305..306)
                                                                        },
                                                                        arguments: None
                                                                    }
                                                                ]
                                                            }
                                                        }
                                                    ),
                                                    position: 1,
                                                    as_token: Some(As),
                                                    gt_token: Gt
                                                }
                                            ),
                                            path: Path {
                                                leading_colon: None,
                                                segments: [
                                                    PathSegment {
                                                        ident: Ident {
                                                            ident: "Trait",
                                                            span: #0 bytes(310..315)
                                                        },
                                                        arguments: None
                                                    },
                                                    Colon2,
                                                    PathSegment {
                                                        ident: Ident {
                                                            ident: "Value",
                                                            span: #0 bytes(318..323)
                                                        },
                                                        arguments: None
                                                    }
                                                ]
                                            }
                                        }
                                    )
                                )
                            ],
                            gt_token: Gt
                        }
                    )
                }
            ]
        }
    }
)
*/




//TODO improve and check logic
fn contains_param_type(param_ident: &Ident, to_check_against: &Type) -> bool {
    //if param_type == to_check_against { return true; }
    match to_check_against {
        Type::Array(type_array) => { contains_param_type(param_ident,&type_array.elem) }
        Type::BareFn(type_bare_fn) => {
            let function_args = &type_bare_fn.inputs;
            for function_arg in function_args {
                if contains_param_type(param_ident, &function_arg.ty) { return true }
            }

            if let syn::ReturnType::Type(_, return_type) = &type_bare_fn.output {
                return contains_param_type(param_ident,&return_type)
            }
            false
        }
        Type::Group(type_group) => { contains_param_type(param_ident,&type_group.elem) }
        Type::ImplTrait(_type_impl_trait) => {false}
        Type::Infer(_type_infer) => { panic!("Can't tell _") }
        Type::Macro(_type_macro) => { panic!("Can't tell as macro") }
        Type::Never(_type_never) => { false}
        Type::Paren(type_paren) => { contains_param_type(param_ident,&type_paren.elem) }
        Type::Path(type_path) => {

            for segments in &type_path.path.segments {

                let seg_ident = &segments.ident;

                if seg_ident == param_ident { return true }

                match &segments.arguments {
                    PathArguments::None => continue,
                    PathArguments::AngleBracketed(args) => {
                        let args = &args.args;
                        for arg in args {
                            match arg {
                            GenericArgument::Lifetime(_lifetime) => continue,
                            GenericArgument::Type(typ) => {
                                if contains_param_type(param_ident, &typ) { return true; }
                            }
                            GenericArgument::Binding(binding) => {
                                if contains_param_type(param_ident, &binding.ty) { return true; }
                            }
                            GenericArgument::Constraint(_constraint) => continue,
                            GenericArgument::Const(_expr) => { panic!("Can't tell as expr") }
                            }
                        }
                    }
                    PathArguments::Parenthesized(args) => {
                        let function_args = &args.inputs;
                        for function_arg in function_args {
                            if contains_param_type(param_ident, &function_arg) { return true }
                        }

                        if let syn::ReturnType::Type(_, return_type) = &args.output {
                            return contains_param_type(param_ident,&return_type)
                        }
                        return false
                    }
                }
            }

            false
        }
        Type::Ptr(type_ptr) => { contains_param_type(param_ident,&type_ptr.elem) }
        Type::Reference(type_reference) => { contains_param_type(param_ident,&type_reference.elem)}
        Type::Slice(type_slice) => { contains_param_type(param_ident,&type_slice.elem) }
        Type::TraitObject(_type_trait_object) => { false }
        Type::Tuple(type_tuple) => {
            for tuple_term_type in &type_tuple.elems {
                if contains_param_type(param_ident,&tuple_term_type) { return true; }
            }
            false
        }
        Type::Verbatim(_token_stream) => { panic!("Can't tell as TokenStream") }
        _ => panic!(),
    }
}














