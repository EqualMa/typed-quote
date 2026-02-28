use std::marker::PhantomData;

use crate::{
    IntoTokenTree, IntoTokens, RefWithSpan, ToTokenTree, ToTokens, WithSpan,
    maybe_span::{MaybeSpan, NoSpan},
    sealed,
};

#[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
use crate::into_st::IntoST;

#[derive(Debug, Clone, Copy)]
pub struct Empty;
mod empty;

mod never;

mod either;

mod option;

// region: group
#[derive(Debug, Clone, Copy)]
pub struct Parenthesis<T, S: MaybeSpan = NoSpan> {
    pub stream: T,
    pub delimiter_span: S,
}
#[derive(Debug, Clone, Copy)]
pub struct Bracket<T, S: MaybeSpan = NoSpan> {
    pub stream: T,
    pub delimiter_span: S,
}
#[derive(Debug, Clone, Copy)]
pub struct Brace<T, S: MaybeSpan = NoSpan> {
    pub stream: T,
    pub delimiter_span: S,
}

mod group;

// endregion
// region: Ident & Lifetime
#[derive(Debug, Clone, Copy)]
pub struct Ident<'a, S: MaybeSpan = NoSpan>(&'a str, S);
mod ident;

/// A leading `'` is included.
#[derive(Debug, Clone, Copy)]
pub struct Lifetime<'a, S: MaybeSpan = NoSpan>(&'a str, S);
mod lifetime;

pub trait HasConstIdent {
    const IDENT: Ident<'static>;
}

pub struct ConstIdent<T: HasConstIdent + ?Sized, S: MaybeSpan = NoSpan>(PhantomData<T>, S);
mod const_ident;

pub trait HasConstLifetime {
    const LIFETIME: Lifetime<'static>;
}

pub struct ConstLifetime<T: HasConstLifetime + ?Sized, S: MaybeSpan = NoSpan>(PhantomData<T>, S);
mod const_lifetime;
// endregion
// region: Concat
#[derive(Debug, Clone, Copy)]
pub struct Concat<A: IntoTokens, B: IntoTokens>(pub A, pub B);
#[derive(Debug, Clone, Copy)]
pub struct ConcatWithDefaultSpan<A: IntoTokens, B: IntoTokens, S: crate::Span>(pub A, pub B, pub S);
#[derive(Debug, Clone, Copy)]
pub struct ConcatWithReplacedSpan<A: IntoTokens, B: IntoTokens, S: crate::Span>(
    pub A,
    pub B,
    pub S,
);
mod concat;
// endregion
// region: iter
#[derive(Debug, Clone, Copy)]
pub struct IterTokens<I: IntoIterator<Item: IntoTokens>>(pub I);
#[derive(Debug, Clone, Copy)]
pub struct IterTokensWithDefaultSpan<I: IntoIterator<Item: IntoTokens>, S: crate::Span>(
    pub I,
    pub S,
);
#[derive(Debug, Clone, Copy)]
pub struct IterTokensWithReplacedSpan<I: IntoIterator<Item: IntoTokens>, S: crate::Span>(
    pub I,
    pub S,
);
mod iter_tokens;
// endregion

/// Sub set of [`const LEGAL_CHARS`](https://doc.rust-lang.org/stable/src/proc_macro/lib.rs.html#959).
pub mod punct;

pub mod puncts;

pub mod __private;
