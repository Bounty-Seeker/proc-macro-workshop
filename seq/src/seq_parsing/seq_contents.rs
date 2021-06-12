use proc_macro2::TokenStream;
use seq_contents_inner::{InnerSeqContent, SeqMode};
use syn::parse::ParseStream;
use syn::Result;
use proc_macro2::Ident;


mod seq_contents_inner;

pub struct ValidatedSeqContents {
    partial_or_whole: SeqMode,
    seq_contents: InnerSeqContent,
}

impl ValidatedSeqContents {

    pub fn create_parser<'a>(id : Ident) -> impl Fn(ParseStream<'a>) -> Result<Self> {

        move |input:ParseStream<'_>| {

            // create parser
            let inner_seq_parser = InnerSeqContent::create_parser(id.clone());

            // initial parse
            let inner_seq_content: InnerSeqContent = inner_seq_parser(input)?;

            // Set initial mode
            let mut mode = None;

            // validate that the tokenstream repeats all or only some sections
            inner_seq_content.validate(&mut mode)?;

            // if initial mode is still None then no repeated groups so set to Whole
            let mode = mode.unwrap_or(SeqMode::Whole);

            Ok(ValidatedSeqContents {
                partial_or_whole: mode,
                seq_contents: inner_seq_content,
            })

        }
    }


    /// generate output token stream
    pub fn generate_output(
        self,
        range: std::ops::Range<u128>,
        ident_to_match: Ident,
    ) -> syn::Result<TokenStream> {
        let mut output = TokenStream::new();

        match self.partial_or_whole {
            SeqMode::Whole => {
                // for each val in range produce output
                for val in range {
                    let out_val = self
                        .seq_contents
                        .generate_output_repeated_whole(val, &ident_to_match)?;
                    output.extend(out_val);
                }
            }
            SeqMode::Partial => {
                // pass range to lower tokens to expand
                let out_total = self
                    .seq_contents
                    .generate_output_repeated_groups(&range, &ident_to_match)?;
                output.extend(out_total);
            }
        }


        Ok(output)
    }
}
