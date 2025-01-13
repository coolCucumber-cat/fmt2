//! formatting stuff
//!
//! TODO:
//! - this documentation and stuff, readme
//! - all documentation
//! - double check project signature (doc alias, doc hidden, naming)
//! - macros to declare structs for formatting not just using
//! - clean up
//! - testing
//! - git and github
//! - reduce repetetion use good style and techniques. more macro recursion instead of repetition
//! - reduce repetition by avoiding polishing work at start (move all resposibility to fmt_write)
//! - make fmt to fmt_advanced with optional args
//! - better naming for macros
//! - reduce repetition in fmt structs (no unit struct, just field struct with no fields)
//! - better name for fmt structs (FmtTemp etc...)
//! - colours and styling (ascii only, no win7 (win10 is almost obsolete, so win7 definitely not worth it either, also no devs use win7))
//! - manage error handling outside of write_fmt_single instead of inside macro
//! - make fmt do everything, including write and generate
//! - flush hint in write
//! - custom fmtable with closure
//! - put all styling in fmt macro
//! - lock stdout and stderr

#![expect(internal_features)]
#![feature(
    never_type,
    unwrap_infallible,
    fmt_internals,
    formatting_options,
    ascii_char,
    // concat_idents,
    // try_blocks,
    // specialization,
    // min_specialization,
    // impl_trait_in_assoc_type
)]
pub mod ansi;
pub mod macros;
pub mod str;
pub mod utils;
pub mod write;
pub mod write_to;
