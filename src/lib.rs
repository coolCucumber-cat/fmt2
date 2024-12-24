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
//! - reduce repetetion and use good style and techniques. more macro recursion instead of repetition
//! - reduce repetition in fmt structs (no unit struct, just field struct with no fields)
//! - better name for fmt structs (FmtTemp etc...)
//! - colours and styling (ascii only, no win7 (win10 is almost obsolete, so win7 definitely not worth it either, also no devs use win7))

#![expect(internal_features)]
#![feature(
    never_type,
    unwrap_infallible,
    fmt_internals,
    formatting_options,
    concat_idents,
    try_blocks,
    // specialization,
    // min_specialization,
    associated_type_defaults
)]
pub mod macros;
pub mod utils;
pub mod write;
pub mod write_to;
