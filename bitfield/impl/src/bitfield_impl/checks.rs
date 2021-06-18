use super::*;
use seq_macro::seq;

const ONES: [&str; 9] = [
"Zero",
"One",
"Two",
"Three",
"Four",
"Five",
"Six",
"Seven",
"Eight",
];


pub fn generate_multiple_of_eight_bits_check() -> Result<TokenStream> {

    let mut output = TokenStream::new();

    let type_magic = quote! (
        pub type IsMultipleOfEightBits<T> = <<T as ModEight>::Marker as TotalSizeIsMultipleOfEightBits>::Check;
    );

    output.extend(type_magic);

    let mod_eight = quote! {
        pub trait ModEight {
            type Marker;
        }
    };

    output.extend(mod_eight);

    seq!( N in 0 ..8 {

        let num_id : &str = ONES[N];

        let marker_id = format_ident!("{}ModEight",num_id);

        let marker_type = quote!(
            pub enum #marker_id {}

            impl ModEight for [(); N] {
                type Marker = #marker_id;
            }
        );

        output.extend(marker_type);
    });


    let final_check = quote!(
        pub trait TotalSizeIsMultipleOfEightBits {
            type Check;
        }

        impl TotalSizeIsMultipleOfEightBits for ZeroModEight{
            type Check = ();
        }
    );

    output.extend(final_check);

    let output = quote!(
        mod bitfields {
            pub mod checks{
            #output
            }
        }
    );

    Ok(output)
}


pub fn generate_multiple_of_eight_run_check(size:&TokenStream) -> Result<TokenStream> {

    let mut output = TokenStream::new();

    let run_check = quote! (
        let _ :  bitfields::checks::IsMultipleOfEightBits<[(); #size % 8]>;
    );

    output.extend(run_check);

    Ok(output)
}