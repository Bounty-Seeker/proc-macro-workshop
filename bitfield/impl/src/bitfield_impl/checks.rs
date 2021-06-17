use super::*;
use seq_macro::seq;

const ONES: [&str; 20] = [
"zero",
"one",
"two",
"three",
"four",
"five",
"six",
"seven",
"eight",
"nine",
"ten",
"eleven",
"twelve",
"thirteen",
"fourteen",
"fifteen",
"sixteen",
"seventeen",
"eighteen",
"nineteen",
];

fn generate_multiple_of_eight_bits_check() -> Result<TokenStream> {

    let mut output = TokenStream::new();

    let type_magic = quote! (
        type IsMultipleOfEightBits<T> = <<T as ModEight>::Marker as MultipleOfEightBits>::Check;
    );

    output.extend(type_magic);

    let mod_eight = quote! {
        trait ModEight {
            type Marker;
        }
    };

    output.extend(mod_eight);

    seq!( N in 0 ..=8 {

        let num_id : &str = ONES[N];

        let marker_id = format_ident!("{}ModEight",num_id);

        let marker_type = quote!(
            enum #marker_id {}

            impl ModEight for [(); N] {
                type Marker = #marker_id;
            }
        );

        output.extend(marker_type);
    });


    let final_check = quote!(
        trait MultipleOfEightBits {
            type Check = ();
        }

        impl MultipleOfEightBits for ZeroModEight{
        }
    );

    output.extend(final_check);

    Ok(output)
}
