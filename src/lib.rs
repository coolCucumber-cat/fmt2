//! formatting stuff
//!
//! TODO:
//! - this documentation and stuff, readme
//! - all documentation
//! - double check project signature (doc alias, doc hidden, naming)
//! - macros to declare structs for formatting not just using
//! - clean up
//! - testing
//! - better name for fmt structs (FmtTemp etc...)
//! - make fmt_advanced impl writeto and make writeto impl fmt (maybe rename fmtadvanced to prefmt)

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(internal_features)]
#![cfg_attr(feature = "never_type", feature(never_type))]
#![cfg_attr(feature = "fmt_internals", feature(fmt_internals))]
#![cfg_attr(feature = "formatting_options", feature(formatting_options))]

pub mod ansi;
pub mod macros;
pub mod screen_area;
pub mod str;
pub mod utils;
pub mod write;
pub mod write_to;
