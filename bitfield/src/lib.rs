// Crates that have the "proc-macro" crate type are only allowed to export
// procedural macros. So we cannot have one crate that defines procedural macros
// alongside other types of public APIs like traits and structs.
//
// For this project we are going to need a #[bitfield] macro but also a trait
// and some structs. We solve this by defining the trait and structs in this
// crate, defining the attribute macro in a separate bitfield-impl crate, and
// then re-exporting the macro from this crate so that users only have one crate
// that they need to import.
//
// From the perspective of a user of this crate, they get all the necessary APIs
// (macro, trait, struct) through the one bitfield crate.
pub use bitfield_impl::{bitfield, BitfieldSpecifier};

use bitfield_impl::generate_checks;
use seq_macro::seq;
use std::convert::TryInto;

// TODO other things

/// Specifer is the trait required for each field's type of the bitflags
pub trait Specifier {
    const BITS : usize;
    type InOutType;

    fn from_u64(inp : u64) -> Self::InOutType;

    fn to_u64(inp : Self::InOutType) -> u64;
}

seq!(N in 1..=8 {

    pub enum B#N {}

    impl Specifier for B#N {
        const BITS : usize = N;
        type InOutType = u8;

        fn from_u64(inp : u64) -> Self::InOutType {
            inp.try_into().unwrap()
        }

        fn to_u64(inp : Self::InOutType) -> u64{
            inp as u64
        }
    }

});

seq!(N in 9..=16 {

    pub enum B#N {}

    impl Specifier for B#N {
        const BITS : usize = N;
        type InOutType = u16;

        fn from_u64(inp : u64) -> Self::InOutType {
            inp.try_into().unwrap()
        }

        fn to_u64(inp : Self::InOutType) -> u64{
            inp as u64
        }
    }

});

seq!(N in 17..=32 {

    pub enum B#N {}

    impl Specifier for B#N {
        const BITS : usize = N;
        type InOutType = u32;

        fn from_u64(inp : u64) -> Self::InOutType {
            inp.try_into().unwrap()
        }

        fn to_u64(inp : Self::InOutType) -> u64{
            inp as u64
        }
    }

});

seq!(N in 33..=64 {

    pub enum B#N {}

    impl Specifier for B#N {
        const BITS : usize = N;
        type InOutType=u64;

        fn from_u64(inp : u64) -> Self::InOutType {
            inp
        }

        fn to_u64(inp : Self::InOutType) -> u64{
            inp as u64
        }
    }

});

impl Specifier for bool {
    const BITS : usize = 1;
    type InOutType = bool;

    fn from_u64(inp : u64) -> Self::InOutType {
        match inp {
            0 => false,
            1 => true,
            _ => unreachable!()
        }
    }

    fn to_u64(inp : Self::InOutType) -> u64{
        inp.into()
    }
}


generate_checks!();