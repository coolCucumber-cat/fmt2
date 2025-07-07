use syn::{Expr, ExprLit, Token, parse::Parse};

use crate::utils::{parse_curly_brackets, parse_round_brackets};

mod kw {
    syn::custom_keyword!(err);
    syn::custom_keyword!(lock);
}

pub struct WriterStd {
    pub err: bool,
    pub lock: bool,
}
impl Parse for WriterStd {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![#]>()?;
        let err = input.parse::<kw::err>().is_ok();
        let lock = input.parse::<kw::lock>().is_ok();
        Ok(Self { err, lock })
    }
}

pub enum Writer {
    Expr(Box<Expr>),
    Std(WriterStd),
}
impl Parse for Writer {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<WriterStd>().map(Writer::Std).or_else(|_| {
            input
                .parse::<Expr>()
                .map(|expr| Writer::Expr(Box::new(expr)))
        })
    }
}

enum Fallibility {
    Fallible,
    Infallible,
}
impl Parse for Fallibility {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input
            .parse::<Token![?]>()
            .map(|_| Fallibility::Fallible)
            .or_else(|_| input.parse::<Token![!]>().map(|_| Fallibility::Infallible))
    }
}

#[derive(Default)]
struct WriterSettings {
    fallibility: Option<Fallibility>,
}
impl Parse for WriterSettings {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![;]>()?;
        let fallibility = input.parse::<Fallibility>().ok();
        Ok(WriterSettings { fallibility })
    }
}

pub struct FmtModeWriter {
    writer: Writer,
    settings: WriterSettings,
}
impl Parse for FmtModeWriter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let writer = input.parse::<Writer>()?;
        let settings = input.parse::<WriterSettings>().unwrap_or_default();
        Ok(Self { writer, settings })
    }
}

pub enum FmtMode {
    Writer(FmtModeWriter),
    String,
}
impl Parse for FmtMode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = parse_round_brackets(input)?;
        let writer = content.parse::<FmtModeWriter>();
        match writer {
            Ok(writer) => Ok(FmtMode::Writer(writer)),
            Err(_) => Ok(FmtMode::String),
        }
    }
}

struct FmtTokenLit(Expr);

impl Parse for FmtTokenLit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
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

enum FmtTokenExprArgs {
    None,
}
impl Parse for FmtTokenExprArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self::None)
    }
}

struct FmtTokenExpr {
    expr: Expr,
    args: FmtTokenExprArgs,
}
impl Parse for FmtTokenExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = parse_curly_brackets(input)?;
        let expr = content.parse::<Expr>()?;
        let args = if content.parse::<Token![;]>().is_ok() {
            content.parse::<FmtTokenExprArgs>()?
        } else {
            FmtTokenExprArgs::None
        };
        Ok(Self { expr, args })
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

pub struct Fmt {
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
