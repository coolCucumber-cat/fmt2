use crate::fmt::stage1;

mod fmt;
mod utils;

#[proc_macro]
pub fn fmt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let fmt = syn::parse_macro_input!(input as stage1::Fmt);
    proc_macro::TokenStream::from(fmt)
}
