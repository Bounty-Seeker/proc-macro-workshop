use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TS2, Span};
use quote::ToTokens;
//use quote::ToTokens;
use syn::{Item, parse_macro_input, ItemEnum, Error, Result};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {

    // Check no args
    if !args.is_empty() {
        let err = Error::new_spanned(TS2::from(args), "Unexpected args")
                                .into_compile_error()
                                .into();
        return err;
    }

    // Parse input as syn::item
    let input_item : Item = parse_macro_input!(input as Item);

    // enum_sorted
    let output = sorted_macro(&input_item);

    // Create output
    match output {
        Ok(()) => input_item.into_token_stream().into(),
        Err(err) => { err.into_compile_error().into() }
    }
}

/// Checks if Item is enum
fn is_enum(item:&Item) -> Result<&ItemEnum> {

    if let Item::Enum(item_enum) = item {
        Ok(item_enum)
    } else {
        let err = Error::new(Span::call_site(), "expected enum or match expression");
        Err(err)
    }
}

/// Checks if enum is sorted
fn enum_is_sorted(enu : &ItemEnum) -> Result<()> {

    // get intial variant
    let prev_ident = match enu.variants.first() {
        Some(prev_vari) => &prev_vari.ident,
        None => return Ok(())
    };

    // check each variant
    for variant in &enu.variants {

        // get variant ident
        let vari_ident = &variant.ident;

        // if previous ident is less than current ident continue
        if prev_ident <= vari_ident {
            continue;
        }

        // now create error as not sorted

        // get ident the misplaced ident should be before
        let mut variants = enu.variants.iter()
        .map(|variant| {
            &variant.ident
        }).skip_while(|cur_ident|  *cur_ident <= vari_ident );

        // Unwrap doesn't panic as we know element with greater ident exists
        let sort_before_ident = variants.next().unwrap();
        //println!("first: {:?}", first);

        // construct error msg
        let err_msg = format!("{} should sort before {}", vari_ident, sort_before_ident);

        // finally construct and return error
        let err = Error::new(vari_ident.span(), err_msg);
        return Err(err);
    }

    Ok(())
}

fn _match_sorted(_input_item : &Item) -> Result<TS2> {

    // Parse input as syn::item
    //let input_item : Item = parse_macro_input!(input as Item);

//.map_err(Error::into_compile_error);

    unimplemented!()
}

fn sorted_macro(input_item: &Item) -> Result<()> {

    // Check if enum
    let input_enum = is_enum(input_item)?;

    // Check if sorted
    enum_is_sorted(input_enum)
}






















