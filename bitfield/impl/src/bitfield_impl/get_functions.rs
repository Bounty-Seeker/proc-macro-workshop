use proc_macro2::TokenStream;
use syn::{Ident, Result, Type};
use quote::quote;
//use std::convert::TryInto;

pub fn create_get_function(get_id : Ident, start_bit: &TokenStream, end_bit : &TokenStream,  field_type : &Type) -> Result<TokenStream> {

    let get_func = quote!(

        fn #get_id (&self) -> <#field_type as Specifier>::InOutType {
        
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
        
            if start_byte == end_byte {
        
                //println!("in here");
        
                // create mask
                let mask = ((2u32.pow(end_offset)) - (2u32.pow(start_offset))) as u8;
        
                //println!("mask {:0>20b}", mask);
        
                // mask and byte then shift
                let output = (mask & self.data[start_byte]) >> start_offset;
        
                let output = output as <#field_type as Specifier>::InOutType;
        
                return output;
            }
        
            // get first bytes
            let mut output : u64 = 0;
        
            let cur_val = self.data[start_byte] >> start_offset;
        
            //println!("cur_val {:0>8b}", cur_val);
        
            output += cur_val as u64;
        
            let mut offset = 8 - start_offset;
        
            // get values of inner bytes
            for ind in inner_range {
        
                // get value
                let cur_val = self.data[ind] as u64;
        
                //println!("cur_val {:0>8b}", cur_val);
        
                
                // shifted value
                let shifted_val : u64 = cur_val << offset;
                //println!("shifted_valz {:0>20b}", shifted_val);
                //println!("offset {:0>8b}", offset);
                //println!("start offset : {:0>08b}", start_offset);
        
        
                
                // add shifted value to output
                output += shifted_val;
                
                // add to offset
                offset += 8;
            }
        
            // return to avoid out of bounds on last field
            if end_offset == 0 {
                let output = output as <#field_type as Specifier>::InOutType;
                return output;
            }
        
            // get last byte value
            let mask : u8 = (2u32.pow(end_offset)-1) as u8;
            //println!("mask {:0>8b}", cur_val);
        
        
            let cur_val = (mask & self.data[end_byte]) as u64;
        
            // shifted value
            let shifted_val : u64 = cur_val << offset;
        
            // add shifted value to output
            output += shifted_val;
        
            let output = output as <#field_type as Specifier>::InOutType;
        
            output
        }
        
    );

    Ok(get_func)
}
