use proc_macro2::TokenStream;
use syn::{Ident, Result};
use quote::quote;

pub fn create_set_function(set_id : Ident, start_bit: &TokenStream, end_bit : &TokenStream) -> Result<TokenStream> {

    let set_func = quote!(
        fn #set_id (&mut self, inp : u64) {
        
            let start : usize = #start_bit;
            let end : usize = #end_bit;
        
            // bytes we start and end at
            let start_byte = start/8;
            let end_byte = end/8;
        
            // range which starts at byte after start_byte and ends at the byte before end_byte.
            let inner_range = (start_byte+1) .. (end_byte);
        
            // start offset which is the bit the data starts at on the start_byte.
            let start_offset = (start % 8) as u32;
        
            // end offset which is the bit after on the end_byte.
            let end_offset = (end % 8) as u32;
        
            // panic if input is too large
            if inp >= 2u64.pow((end-start) as u32) { panic!("Given input is too large"); }
            //println!("start : {}", start);
        
            if start_byte == end_byte {
        
                // create masks
                let mask : u64 = 2u64.pow(end_offset - start_offset) - 1;

                //println!("mask {:0>8b}", mask as u8);
        
                // mask input then shift
                let cur_input : u8 = ((mask & inp) << start_offset) as u8;

                //println!("masked input {:0>8b}", cur_input as u8);
        
                // to set bits before and after to 1
                let second_mask : u8 = ((2u64.pow(start_offset) - 1) + (2u64.pow(8) - 2u64.pow(end_offset))) as u8;

                //println!("second mask {:0>8b}", second_mask);
        
                // current data in byte to keep
                let data_masked : u8 = second_mask & self.data[start_byte];

                //println!("data masked {:0>8b}", data_masked);
        
                // set bits
                self.data[start_byte] = data_masked + cur_input;
        
                return;
            }
        
        
            //println!("input {:0>23b}", inp);

            let mask : u64 = 2u64.pow(8 - start_offset) - 1;

            //println!("amask {:0>8b}", mask as u8);
        
            // mask input then shift
            let cur_input : u8 = ((mask & inp) << start_offset) as u8;

            //println!("masked input {:0>8b}", cur_input);
        
            // to set bits before and after to 1
            let second_mask : u8 = (2u64.pow(start_offset) - 1) as u8;

            //println!("second mask {:0>8b}", second_mask);

            // current data in byte to keep
            let data_masked = self.data[start_byte] & second_mask;

            //println!("alt input {:0>8b}", data_masked);

            // set bits
            self.data[start_byte] = data_masked + cur_input;
        
            //println!("data a: {:0>8b}", self.data[start_byte]);
        
            // proceed to inner bytes
        
            // bit in input that goes after the top bit that appears in data[ind]
            let mut cur_upper_bit : u32 = 16-start_offset;
        
            // bit in input that goes that first appears in data[ind]
            let mut cur_lower_bit : u32 = 8-start_offset;
        
            // get values of inner bytes
            for ind in inner_range {
        
                let mask : u64 = (2u128.pow(cur_upper_bit) - 2u128.pow(cur_lower_bit)) as u64;

                //println!("mask {:0>24b}", mask);
                //println!("inpu {:0>24b}", inp);


                // mask input then shift
                let cur_input : u64 = mask & inp;
                //println!("cur_inp {:0>24b}", cur_input);

                let shifted_input : u8 = (cur_input >> cur_lower_bit) as u8;
                
                // set bits
                self.data[ind] = shifted_input;
                //println!("data loop {:0>8b}", self.data[ind]);

        
                // update cur_bits
                cur_upper_bit += 8;
        
                cur_lower_bit += 8;
            }
        
            // return to avoid out of bounds on last field
            if end_offset == 0 { return; }

            //println!("here");

            let mask : u64 = (2u128.pow((end-start) as u32) - 2u128.pow(cur_lower_bit)) as u64;
            //println!("mask {:0>24b}", mask);
            //println!("inpu {:0>24b}", inp);
        
            // mask input then shift
            let cur_input : u64 = mask & inp;
            //println!("cuin {:0>24b}", cur_input);

            let shifted_input : u8 = (cur_input >> cur_lower_bit) as u8;
        
            // mask to set other bits to 1
            let second_mask = !((mask >> cur_lower_bit) as u8);

            //println!("second mask {:0>8b}", second_mask);

            // current data in byte to keep
            let data_masked = self.data[end_byte] & second_mask;

            //println!("alt input {:0>8b}", data_masked);

            // set bits
            self.data[end_byte] = data_masked + shifted_input;
        
            //println!("data a: {:0>8b}", self.data[end_byte]);
        
        }

    );

    Ok(set_func)
}

