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

#[derive(Default)]
pub enum Fallibility {
    Fallible,
    Infallible,
    #[default]
    Ignore,
}
impl Parse for Fallibility {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input
            .parse::<Token![?]>()
            .map(|_| Fallibility::Fallible)
            .or_else(|_| input.parse::<Token![!]>().map(|_| Fallibility::Infallible))
            .or(Ok(Fallibility::Ignore))
    }
}

#[derive(Default)]
pub struct WriterArgs {
    fallibility: Fallibility,
}
impl Parse for WriterArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![;]>()?;
        let fallibility = input.parse::<Fallibility>()?;
        Ok(WriterArgs { fallibility })
    }
}

pub struct FmtModeWriter {
    writer: Writer,
    writer_args: WriterArgs,
}
impl Parse for FmtModeWriter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let writer = input.parse::<Writer>()?;
        let writer_args = input.parse::<WriterArgs>().unwrap_or_default();
        Ok(Self {
            writer,
            writer_args,
        })
    }
}

pub enum FmtMode {
    Writer(FmtModeWriter),
    String,
}
impl Parse for FmtMode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = parse_round_brackets(input)?;
        content
            .parse::<FmtModeWriter>()
            .map(FmtMode::Writer)
            .or(Ok(FmtMode::String))
    }
}

pub struct FmtTokenLit(Expr);
impl Parse for FmtTokenLit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input
            .parse::<ExprLit>()
            .map(|lit| Self(Expr::Lit(lit)))
            .or_else(|_| {
                let content = parse_round_brackets(input)?;
                let expr = content.parse::<Expr>()?;
                Ok(Self(expr))
            })
    }
}

#[derive(Default)]
pub enum ExprMode {
    #[default]
    None,
}
impl Parse for ExprMode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self::None)
    }
}

#[derive(Default)]
pub struct FmtTokenExprArgs {
    expr_mode: ExprMode,
}
impl Parse for FmtTokenExprArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![;]>()?;
        let expr_mode = input.parse::<ExprMode>()?;
        Ok(Self { expr_mode })
    }
}

pub struct FmtTokenExpr {
    expr: Box<Expr>,
    args: FmtTokenExprArgs,
}
impl Parse for FmtTokenExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = parse_curly_brackets(input)?;
        let expr = content.parse::<Expr>()?;
        let args = content.parse::<FmtTokenExprArgs>().unwrap_or_default();
        Ok(Self {
            expr: Box::new(expr),
            args,
        })
    }
}

pub enum FmtToken {
    Lit(Vec<FmtTokenLit>),
    Expr(FmtTokenExpr),
}

pub struct FmtTokens {
    tokens: Vec<FmtToken>,
}
impl Parse for FmtTokens {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // let tokens = input;
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
