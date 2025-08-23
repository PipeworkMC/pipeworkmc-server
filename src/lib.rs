#![feature(

    // Language
    decl_macro,
    never_type,

    // Standard library
    maybe_uninit_array_assume_init,
    mpmc_channel

)]


pub mod conn;

pub mod util;


pub const PROTOCOL : u32 = 772;
