use quote::quote;

use super::stage1;

impl From<stage1::Writer> for proc_macro::TokenStream {
    fn from(value: stage1::Writer) -> Self {
        match value {
            stage1::Writer::Expr(expr) => quote! { #expr },
            stage1::Writer::Std(stage1::WriterStd {
                err: false,
                lock: false,
            }) => quote! { ::std::io::stdout() },
            stage1::Writer::Std(stage1::WriterStd {
                err: true,
                lock: false,
            }) => quote! { ::std::io::stderr() },
            stage1::Writer::Std(stage1::WriterStd {
                err: false,
                lock: true,
            }) => quote! { ::std::io::stdout().lock() },
            stage1::Writer::Std(stage1::WriterStd {
                err: true,
                lock: true,
            }) => quote! { ::std::io::stderr().lock() },
        }
        .into()
    }
}

impl From<stage1::Fmt> for proc_macro::TokenStream {
    fn from(value: stage1::Fmt) -> Self {
        quote! {}.into()
    }
}
