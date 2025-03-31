use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, ExprLit, Ident, Lifetime, Lit, Token,
    parse::{Parse, ParseBuffer, ParseStream},
};

mod kw {
    syn::custom_keyword!(str);
    syn::custom_keyword!(err);
    syn::custom_keyword!(lock);
}

enum Writer {
    Expr(Expr),
    Std { err: bool, lock: bool },
}

impl From<Writer> for TokenStream {
    fn from(value: Writer) -> Self {
        match value {
            Writer::Expr(expr) => quote! { #expr },
            Writer::Std {
                err: false,
                lock: false,
            } => quote! { ::std::io::stdout() },
            Writer::Std {
                err: true,
                lock: false,
            } => quote! { ::std::io::stderr() },
            Writer::Std {
                err: false,
                lock: true,
            } => quote! { ::std::io::stdout().lock() },
            Writer::Std {
                err: true,
                lock: true,
            } => quote! { ::std::io::stderr().lock() },
        }
        .into()
    }
}

enum FmtModeInternal {
    Internal { lifetime: Lifetime },
    None,
}

impl Parse for FmtModeInternal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<Lifetime>() {
            Ok(lifetime) => Ok(Self::Internal { lifetime }),
            Err(_) => Ok(Self::None),
        }
    }
}

enum FmtMode {
    Write {
        writer: Writer,
        fallible: bool,
        internal: FmtModeInternal,
    },
    ToString,
}

impl Parse for FmtMode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = parse_round_brackets(input)?;

        if content.is_empty() {
            return Ok(Self::ToString);
        }

        let internal = content.parse::<FmtModeInternal>()?;

        let fallible = content.parse::<Token![?]>().is_ok();

        let writer = if content.parse::<Token![#]>().is_ok() {
            let err = content.parse::<kw::err>().is_ok();
            let lock = content.parse::<kw::lock>().is_ok();
            Writer::Std { err, lock }
        } else {
            let writer = content.parse::<Expr>()?;
            Writer::Expr(writer)
        };

        Ok(Self::Write {
            writer,
            fallible,
            internal,
        })
    }
}

struct FmtTokenLit(Expr);

impl Parse for FmtTokenLit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(lit) = input.parse::<ExprLit>() {
            let expr = Expr::Lit(lit);
            Ok(Self(expr))
        } else {
            let content = parse_round_brackets(input)?;
            let expr = content.parse::<Expr>()?;
            Ok(Self(expr))
        }
    }
}

struct FmtTokenExpr {
    expr: Expr,
}

impl Parse for FmtTokenExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content = parse_curly_brackets(input)?;
        let expr = content.parse::<Expr>()?;
    }
}

enum FmtToken {
    Lit(Vec<FmtTokenLit>),
    Expr(FmtTokenExpr),
}

struct FmtTokens {
    tokens: Vec<FmtToken>,
}

impl Parse for FmtTokens {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let tokens = vec![];

        Ok(Self { tokens })
    }
}

struct Fmt {
    mode: FmtMode,
    tokens: FmtTokens,
}

impl Parse for Fmt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mode = input.parse::<FmtMode>()?;
        input.parse::<Token![=>]>()?;
        let tokens = input.parse::<FmtTokens>()?;
        Ok(Self { mode, tokens })
    }
}

impl From<Fmt> for TokenStream {
    fn from(value: Fmt) -> Self {
        quote! {}.into()
    }
}

#[proc_macro]
pub fn fmt(input: TokenStream) -> TokenStream {
    let fmt = syn::parse_macro_input!(input as Fmt);
    TokenStream::from(fmt)
}

fn parse_curly_brackets(input: ParseStream) -> syn::Result<ParseBuffer> {
    Ok(syn::__private::parse_braces(input)?.content)
}
fn parse_square_brackets(input: ParseStream) -> syn::Result<ParseBuffer> {
    Ok(syn::__private::parse_brackets(input)?.content)
}
fn parse_round_brackets(input: ParseStream) -> syn::Result<ParseBuffer> {
    Ok(syn::__private::parse_parens(input)?.content)
}
