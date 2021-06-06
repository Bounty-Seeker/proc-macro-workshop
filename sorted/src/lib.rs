use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TS2, Span};
use quote::ToTokens;
//use quote::ToTokens;
use syn::{Item, parse_macro_input, ItemEnum, Error, Result};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {

    // Check no args
    if !args.is_empty() {
        let err = Error::new(Span::call_site(), "Unexpected args")
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
        let err = Error::new(Span::mixed_site(), "expected enum or match expression");
        Err(err)
    }
}

/// Checks if enum is sorted
fn _enum_is_sorted(_enu : &ItemEnum) -> Result<()> {
    unimplemented!()
}

fn _match_sorted(_input_item : &Item) -> Result<TS2> {

    // Parse input as syn::item
    //let input_item : Item = parse_macro_input!(input as Item);

//.map_err(Error::into_compile_error);

    unimplemented!()
}

fn sorted_macro(input_item: &Item) -> Result<()> {

    // Check if enum
    let _input_enum = is_enum(input_item)?;

    //match Ok

    Ok(())
}






















