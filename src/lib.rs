#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// https://github.com/rust-lang/rust-analyzer/pull/14561
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// cargo build to generate this
mod bindings;

// Reexport generated bindings, so that RA works.
pub use bindings::*;
