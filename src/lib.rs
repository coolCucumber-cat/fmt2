#![expect(internal_features)]
#![feature(
    never_type,
    unwrap_infallible,
    fmt_internals,
    formatting_options,
    concat_idents,
    // specialization,
    // min_specialization,
    associated_type_defaults
)]
pub mod macros;
pub mod utils;
pub mod writable;
pub mod write;
