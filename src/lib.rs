#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(any(
    all(feature = "proc-macro", feature = "proc-macro2"),
    not(doctest),
), doc = include_str!("../README.md"))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "proc-macro")]
extern crate proc_macro;

#[cfg(feature = "proc-macro")]
use proc_macro::{TokenStream, TokenTree};

#[cfg(feature = "proc-macro2")]
use proc_macro2::{TokenStream as TokenStream2, TokenTree as TokenTree2};

/// Into token stream.
///
/// *This trait is sealed and all methods are provided.*
pub trait IntoTokens: sealed::IntoTokens {
    #[cfg(feature = "proc-macro")]
    fn into_tokens(self, tokens: &mut TokenStream);
    #[cfg(feature = "proc-macro")]
    fn into_token_stream(self) -> TokenStream;

    #[cfg(feature = "proc-macro2")]
    fn into_tokens2(self, tokens: &mut TokenStream2);
    #[cfg(feature = "proc-macro2")]
    fn into_token_stream2(self) -> TokenStream2;

    #[doc(hidden)]
    #[cfg(feature = "alloc")]
    #[cfg(feature = "proc-macro")]
    fn box_into_tokens(self: ::alloc::boxed::Box<Self>, tokens: &mut TokenStream);
    #[doc(hidden)]
    #[cfg(feature = "alloc")]
    #[cfg(feature = "proc-macro")]
    fn box_into_token_stream(self: ::alloc::boxed::Box<Self>) -> TokenStream;

    #[doc(hidden)]
    #[cfg(feature = "alloc")]
    #[cfg(feature = "proc-macro2")]
    fn box_into_tokens2(self: ::alloc::boxed::Box<Self>, tokens: &mut TokenStream2);
    #[doc(hidden)]
    #[cfg(feature = "alloc")]
    #[cfg(feature = "proc-macro2")]
    fn box_into_token_stream2(self: ::alloc::boxed::Box<Self>) -> TokenStream2;
}

/// To token stream.
///
/// *This trait is sealed and all methods are provided.*
pub trait ToTokens: IntoTokens + sealed::ToTokens {
    #[cfg(feature = "proc-macro")]
    fn to_tokens(&self, tokens: &mut TokenStream);
    #[cfg(feature = "proc-macro")]
    fn to_token_stream(&self) -> TokenStream;

    #[cfg(feature = "proc-macro2")]
    fn to_tokens2(&self, tokens: &mut TokenStream2);
    #[cfg(feature = "proc-macro2")]
    fn to_token_stream2(&self) -> TokenStream2;
}

impl<T: ?Sized + ToTokens> sealed::ToTokens for &T {}
impl<T: ?Sized + ToTokens> ToTokens for &T {
    #[cfg(feature = "proc-macro")]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        T::to_tokens(self, tokens)
    }

    #[cfg(feature = "proc-macro")]
    fn to_token_stream(&self) -> TokenStream {
        T::to_token_stream(self)
    }

    #[cfg(feature = "proc-macro2")]
    fn to_tokens2(&self, tokens: &mut TokenStream2) {
        T::to_tokens2(self, tokens)
    }

    #[cfg(feature = "proc-macro2")]
    fn to_token_stream2(&self) -> TokenStream2 {
        T::to_token_stream2(self)
    }
}
impl<T: ?Sized + ToTokens> sealed::IntoTokens for &T {}
impl<T: ?Sized + ToTokens> IntoTokens for &T {
    #[cfg(feature = "proc-macro")]
    fn into_tokens(self, tokens: &mut TokenStream) {
        T::to_tokens(self, tokens)
    }
    #[cfg(feature = "proc-macro")]
    fn into_token_stream(self) -> TokenStream {
        T::to_token_stream(self)
    }
    #[cfg(feature = "proc-macro2")]
    fn into_tokens2(self, tokens: &mut TokenStream2) {
        T::to_tokens2(self, tokens)
    }
    #[cfg(feature = "proc-macro2")]
    fn into_token_stream2(self) -> TokenStream2 {
        T::to_token_stream2(self)
    }

    crate::impl_box_into_tokens! {}
}

/// Into a token tree.
///
/// *This trait is sealed and all methods are provided.*
pub trait IntoTokenTree: IntoTokens + sealed::IntoTokenTree {
    #[cfg(feature = "proc-macro")]
    fn into_token_tree(self) -> TokenTree;
    #[cfg(feature = "proc-macro2")]
    fn into_token_tree2(self) -> TokenTree2;

    #[doc(hidden)]
    #[cfg(feature = "alloc")]
    #[cfg(feature = "proc-macro")]
    fn box_into_token_tree(self: ::alloc::boxed::Box<Self>) -> TokenTree;
    #[doc(hidden)]
    #[cfg(feature = "alloc")]
    #[cfg(feature = "proc-macro2")]
    fn box_into_token_tree2(self: ::alloc::boxed::Box<Self>) -> TokenTree2;
}

/// To a token tree.
///
/// *This trait is sealed and all methods are provided.*
pub trait ToTokenTree: IntoTokenTree + ToTokens + sealed::ToTokenTree {
    #[cfg(feature = "proc-macro")]
    fn to_token_tree(&self) -> TokenTree;
    #[cfg(feature = "proc-macro2")]
    fn to_token_tree2(&self) -> TokenTree2;
}

impl<T: ?Sized + ToTokenTree> sealed::ToTokenTree for &T {}
impl<T: ?Sized + ToTokenTree> ToTokenTree for &T {
    #[cfg(feature = "proc-macro")]
    fn to_token_tree(&self) -> TokenTree {
        T::to_token_tree(self)
    }

    #[cfg(feature = "proc-macro2")]
    fn to_token_tree2(&self) -> TokenTree2 {
        T::to_token_tree2(self)
    }
}

impl<T: ?Sized + ToTokenTree> sealed::IntoTokenTree for &T {}
impl<T: ?Sized + ToTokenTree> IntoTokenTree for &T {
    #[cfg(feature = "proc-macro")]
    fn into_token_tree(self) -> TokenTree {
        T::to_token_tree(self)
    }
    #[cfg(feature = "proc-macro2")]
    fn into_token_tree2(self) -> TokenTree2 {
        T::to_token_tree2(self)
    }

    #[cfg(feature = "alloc")]
    #[cfg(feature = "proc-macro")]
    fn box_into_token_tree(self: ::alloc::boxed::Box<Self>) -> TokenTree {
        T::to_token_tree(&self)
    }
    #[cfg(feature = "alloc")]
    #[cfg(feature = "proc-macro2")]
    fn box_into_token_tree2(self: ::alloc::boxed::Box<Self>) -> TokenTree2 {
        T::to_token_tree2(&self)
    }
}

/// A [`proc_macro::Span`] or [`proc_macro2::Span`].
///
/// *This trait is sealed.*
pub trait Span: sealed::Span + Copy + maybe_span::MaybeSpan {}

/// Into tokens with new span.
///
/// *This trait is sealed and all methods are provided.*
pub trait WithSpan: IntoTokens + sealed::WithSpan {
    type WithDefaultSpan<S: Span>: IntoTokens + WithSpan;

    fn with_default_span<S: Span>(self, span: S) -> Self::WithDefaultSpan<S>;

    type WithReplacedSpan<S: Span>: IntoTokens + WithSpan;

    fn with_replaced_span<S: Span>(self, span: S) -> Self::WithReplacedSpan<S>;
}

/// To tokens with new span.
///
/// *This trait is sealed and all methods are provided.*
pub trait RefWithSpan: WithSpan + ToTokens + sealed::RefWithSpan {
    type RefWithDefaultSpan<'a, S: Span>: ToTokens + Copy + RefWithSpan
    where
        Self: 'a;

    fn ref_with_default_span<S: Span>(&self, span: S) -> Self::RefWithDefaultSpan<'_, S>;

    type RefWithReplacedSpan<'a, S: Span>: ToTokens + Copy + RefWithSpan
    where
        Self: 'a;

    fn ref_with_replaced_span<S: Span>(&self, span: S) -> Self::RefWithReplacedSpan<'_, S>;
}

impl<T: ?Sized + RefWithSpan> sealed::RefWithSpan for &T {}
impl<'this, T: ?Sized + RefWithSpan> RefWithSpan for &'this T {
    type RefWithDefaultSpan<'a, S: Span>
        = T::RefWithDefaultSpan<'this, S>
    where
        Self: 'a;

    fn ref_with_default_span<S: Span>(&self, span: S) -> Self::RefWithDefaultSpan<'_, S> {
        T::ref_with_default_span(*self, span)
    }

    type RefWithReplacedSpan<'a, S: Span>
        = T::RefWithReplacedSpan<'this, S>
    where
        Self: 'a;

    fn ref_with_replaced_span<S: Span>(&self, span: S) -> Self::RefWithReplacedSpan<'_, S> {
        T::ref_with_replaced_span(*self, span)
    }
}

impl<T: ?Sized + RefWithSpan> sealed::WithSpan for &T {}
impl<'a, T: ?Sized + RefWithSpan> WithSpan for &'a T {
    type WithDefaultSpan<S: Span> = T::RefWithDefaultSpan<'a, S>;

    fn with_default_span<S: Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        T::ref_with_default_span(self, span)
    }

    type WithReplacedSpan<S: Span> = T::RefWithReplacedSpan<'a, S>;

    fn with_replaced_span<S: Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        T::ref_with_replaced_span(self, span)
    }
}

mod sealed {
    #[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
    use crate::replace_span_of::ReplaceSpanOf;

    #[doc(hidden)]
    pub trait IntoTokens {}
    #[doc(hidden)]
    pub trait ToTokens {}

    #[doc(hidden)]
    pub trait IntoTokenTree {}
    #[doc(hidden)]
    pub trait ToTokenTree {}

    #[doc(hidden)]
    pub trait MaybeSpan {}

    #[cfg(feature = "proc-macro")]
    #[cfg(feature = "proc-macro2")]
    #[doc(hidden)]
    pub trait Span:
        ReplaceSpanOf<proc_macro::TokenStream>
        + ReplaceSpanOf<proc_macro::TokenTree>
        + ReplaceSpanOf<proc_macro::Group>
        + ReplaceSpanOf<proc_macro::Ident>
        + ReplaceSpanOf<proc_macro::Punct>
        + ReplaceSpanOf<proc_macro::Literal>
        + ReplaceSpanOf<proc_macro2::TokenStream>
        + ReplaceSpanOf<proc_macro2::TokenTree>
        + ReplaceSpanOf<proc_macro2::Group>
        + ReplaceSpanOf<proc_macro2::Ident>
        + ReplaceSpanOf<proc_macro2::Punct>
        + ReplaceSpanOf<proc_macro2::Literal>
    {
    }

    #[cfg(feature = "proc-macro")]
    #[cfg(not(feature = "proc-macro2"))]
    #[doc(hidden)]
    pub trait Span:
        ReplaceSpanOf<proc_macro::TokenStream>
        + ReplaceSpanOf<proc_macro::TokenTree>
        + ReplaceSpanOf<proc_macro::Group>
        + ReplaceSpanOf<proc_macro::Ident>
        + ReplaceSpanOf<proc_macro::Punct>
        + ReplaceSpanOf<proc_macro::Literal>
    {
    }

    #[cfg(not(feature = "proc-macro"))]
    #[cfg(feature = "proc-macro2")]
    #[doc(hidden)]
    pub trait Span:
        ReplaceSpanOf<proc_macro2::TokenStream>
        + ReplaceSpanOf<proc_macro2::TokenTree>
        + ReplaceSpanOf<proc_macro2::Group>
        + ReplaceSpanOf<proc_macro2::Ident>
        + ReplaceSpanOf<proc_macro2::Punct>
        + ReplaceSpanOf<proc_macro2::Literal>
    {
    }

    #[cfg(not(feature = "proc-macro"))]
    #[cfg(not(feature = "proc-macro2"))]
    #[doc(hidden)]
    pub trait Span {}

    #[doc(hidden)]
    pub trait WithSpan {}
    #[doc(hidden)]
    pub trait RefWithSpan {}
}

#[macro_export]
macro_rules! quote {
    (#$quoted:ident) => {
        $quoted
    };
    (#$quoted:ident $($rest:tt)+) => {
        $crate::tokens::Concat(
            $quoted,
            $crate::quote!($($rest)+),
        )
    };
    () => {
        $crate::tokens::Empty
    };
    ($only:tt) => {
        $crate::quote_token!($only)
    };
    ($head:tt $($tail:tt)+) => {
        $crate::tokens::Concat(
            $crate::quote_token!($head),
            $crate::quote!($($tail)+),
        )
    };
}

#[doc(no_inline)]
pub use quote as typed_quote;

// https://docs.rs/quote/latest/src/quote/lib.rs.html#1016
#[macro_export]
macro_rules! quote_token {
    ($ident:ident) => {const {
        enum Ident {}
        impl $crate::tokens::HasConstIdent for Ident {
            const IDENT: $crate::tokens::Ident<'static> =
                $crate::tokens::__private::ident($crate::tokens::__private::stringify!($ident));
        }
        $crate::tokens::ConstIdent::<Ident>::new()
    }};
    (::) => {
        $crate::tokens::puncts::Colon2($crate::maybe_span::NoSpan)
    };
    (( $($content:tt)* )) => {
        $crate::tokens::Parenthesis {
            stream: $crate::quote!($($content)*),
            delimiter_span: $crate::maybe_span::NoSpan,
        }
    };
    ([ $($content:tt)* ]) => {
        $crate::tokens::Bracket {
            stream: $crate::quote!($($content)*),
            delimiter_span: $crate::maybe_span::NoSpan,
        }
    };
    ({ $($content:tt)* }) => {
        $crate::tokens::Brace {
            stream: $crate::quote!($($content)*),
            delimiter_span: $crate::maybe_span::NoSpan,
        }
    };
    ($lit:literal) => {const {
        enum Literal {}
        impl $crate::tokens::HasConstLiteral for Literal {
            const LITERAL: $crate::tokens::Literal<'static> =
                $crate::tokens::__private::literal($crate::tokens::__private::stringify!($lit));
        }
        $crate::tokens::ConstLiteral::<Literal>::new()
    }};
    (#) => {
        $crate::tokens::punct::Pound($crate::maybe_span::NoSpan)
    };
    (,) => {
        $crate::tokens::punct::Comma($crate::maybe_span::NoSpan)
    };
    (.) => {
        $crate::tokens::punct::Dot($crate::maybe_span::NoSpan)
    };
    (;) => {
        $crate::tokens::punct::Semi($crate::maybe_span::NoSpan)
    };
    (:) => {
        $crate::tokens::punct::Colon($crate::maybe_span::NoSpan)
    };
    (+) => {
        $crate::tokens::punct::Add($crate::maybe_span::NoSpan)
    };
    (+=) => {
        $crate::tokens::puncts::AddEq($crate::maybe_span::NoSpan)
    };
    (&) => {
        $crate::tokens::punct::And($crate::maybe_span::NoSpan)
    };
    (&&) => {
        $crate::tokens::puncts::AndAnd($crate::maybe_span::NoSpan)
    };
    (&=) => {
        $crate::tokens::puncts::AndEq($crate::maybe_span::NoSpan)
    };
    (@) => {
        $crate::tokens::punct::At($crate::maybe_span::NoSpan)
    };
    (!) => {
        $crate::tokens::punct::Bang($crate::maybe_span::NoSpan)
    };
    (^) => {
        $crate::tokens::punct::Caret($crate::maybe_span::NoSpan)
    };
    (^=) => {
        $crate::tokens::puncts::CaretEq($crate::maybe_span::NoSpan)
    };
    (/) => {
        $crate::tokens::punct::Div($crate::maybe_span::NoSpan)
    };
    (/=) => {
        $crate::tokens::puncts::DivEq($crate::maybe_span::NoSpan)
    };
    (..) => {
        $crate::tokens::puncts::Dot2($crate::maybe_span::NoSpan)
    };
    (...) => {
        $crate::tokens::puncts::Dot3($crate::maybe_span::NoSpan)
    };
    (..=) => {
        $crate::tokens::puncts::DotDotEq($crate::maybe_span::NoSpan)
    };
    (=) => {
        $crate::tokens::punct::Eq($crate::maybe_span::NoSpan)
    };
    (==) => {
        $crate::tokens::puncts::EqEq($crate::maybe_span::NoSpan)
    };
    (>=) => {
        $crate::tokens::puncts::Ge($crate::maybe_span::NoSpan)
    };
    (>) => {
        $crate::tokens::punct::Gt($crate::maybe_span::NoSpan)
    };
    (<=) => {
        $crate::tokens::puncts::Le($crate::maybe_span::NoSpan)
    };
    (<) => {
        $crate::tokens::punct::Lt($crate::maybe_span::NoSpan)
    };
    (*=) => {
        $crate::tokens::puncts::MulEq($crate::maybe_span::NoSpan)
    };
    (!=) => {
        $crate::tokens::puncts::Ne($crate::maybe_span::NoSpan)
    };
    (|) => {
        $crate::tokens::punct::Or($crate::maybe_span::NoSpan)
    };
    (|=) => {
        $crate::tokens::puncts::OrEq($crate::maybe_span::NoSpan)
    };
    (||) => {
        $crate::tokens::puncts::OrOr($crate::maybe_span::NoSpan)
    };
    (?) => {
        $crate::tokens::punct::Question($crate::maybe_span::NoSpan)
    };
    (->) => {
        $crate::tokens::puncts::RArrow($crate::maybe_span::NoSpan)
    };
    (<-) => {
        $crate::tokens::puncts::LArrow($crate::maybe_span::NoSpan)
    };
    (%) => {
        $crate::tokens::punct::Rem($crate::maybe_span::NoSpan)
    };
    (%=) => {
        $crate::tokens::puncts::RemEq($crate::maybe_span::NoSpan)
    };
    (=>) => {
        $crate::tokens::puncts::FatArrow($crate::maybe_span::NoSpan)
    };
    (<<) => {
        $crate::tokens::puncts::Shl($crate::maybe_span::NoSpan)
    };
    (<<=) => {
        $crate::tokens::puncts::ShlEq($crate::maybe_span::NoSpan)
    };
    (>>) => {
        $crate::tokens::puncts::Shr($crate::maybe_span::NoSpan)
    };
    (>>=) => {
        $crate::tokens::puncts::ShrEq($crate::maybe_span::NoSpan)
    };
    (*) => {
        $crate::tokens::punct::Star($crate::maybe_span::NoSpan)
    };
    (-) => {
        $crate::tokens::punct::Sub($crate::maybe_span::NoSpan)
    };
    (-=) => {
        $crate::tokens::puncts::SubEq($crate::maybe_span::NoSpan)
    };
    ($lifetime:lifetime) => {const {
        enum Lifetime {}

        impl $crate::tokens::HasConstLifetime for Lifetime {
            const LIFETIME: $crate::tokens::Lifetime<'static> =
                $crate::tokens::__private::lifetime($crate::tokens::__private::stringify!($lifetime));
        }

        $crate::tokens::ConstLifetime::<Lifetime>::new()
    }};
    (_) => {
        $crate::tokens::ConstIdent::UNDERSCORE
    };
    ($) => {
        $crate::tokens::punct::Dollar($crate::maybe_span::NoSpan)
    };
    (~) => {
        $crate::tokens::punct::Tilde($crate::maybe_span::NoSpan)
    };
}

pub mod maybe_span;

pub mod tokens;

#[derive(Debug, Clone, Copy)]
pub enum Never {}

#[derive(Debug, Clone, Copy)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

pub mod prelude;

macro_rules! impl_many {
    (
        impl<__> $Trait:ident for each_of![
            $($ForTy:ty),+ $(,)?
        ] $impl_body:tt
    ) => {
        $(
            impl $Trait for $ForTy
            $impl_body
        )+
    };
    (
        impl<__> $Trait:ident for each_of_with_generics![
            $([$($generics:tt)*] $ForTy:ty),+ $(,)?
        ] $impl_body:tt
    ) => {
        $(
            impl<$($generics)*> $Trait for $ForTy
            $impl_body
        )+
    };
    ({
        $defs:tt
        $($imps:tt)*
    }) => {
        crate::impl_many! {
            @__defs
            $defs
            {$($imps)*}
        }
    };
    (@__defs { $($(#$def_attr:tt)* {$($defs:tt)*})+ } $imps:tt) => {
        $(
            $(#$def_attr)*
            const _: () = {
                $($defs)*

                crate::impl_many! {
                    @__unwrap $imps
                }
            };
        )+
    };
    (@__unwrap {$($t:tt)*}) => {
        $($t)*
    }
}

macro_rules! impl_to_tokens {
    (copy) => {
        crate::impl_to_tokens! {
            |self, ts| <(Self, &mut _) as crate::into_st::IntoST<()>>::into_st((*self, ts)),
            <Self as crate::into_st::IntoST<_>>::into_st(*self),
        }
    };
    (tt) => {
        #[cfg(feature = "proc-macro")]
        fn to_tokens(&self, tokens: &mut ::proc_macro::TokenStream) {
            tokens.extend(Some(self.to_token_tree()));
        }
        #[cfg(feature = "proc-macro")]
        fn to_token_stream(&self) -> ::proc_macro::TokenStream {
            self.to_token_tree().into()
        }

        #[cfg(feature = "proc-macro2")]
        fn to_tokens2(&self, tokens: &mut ::proc_macro2::TokenStream) {
            tokens.extend(Some(self.to_token_tree2()));
        }
        #[cfg(feature = "proc-macro2")]
        fn to_token_stream2(&self) -> ::proc_macro2::TokenStream {
            self.to_token_tree2().into()
        }
    };
    (#[proxy] |$self_:ident| $proxy:expr) => {
        crate::impl_to_tokens! {
            |$self_, ts| crate::into_st::IntoST::<()>::into_st(($proxy, ts)),
            crate::into_st::IntoST::<_>::into_st($proxy),
        }
    };
    (
        |$self_:ident, $tokens:pat_param $(,)?| $to_tokens:expr
        $(, $to:expr)?
        $(,)?
    ) => {
        #[cfg(feature = "proc-macro")]
        fn to_tokens(&$self_, $tokens: &mut ::proc_macro::TokenStream) {
            #[allow(unused_imports)]
            use ::proc_macro as pm;

            $to_tokens
        }

        #[cfg(feature = "proc-macro")]
        fn to_token_stream(&$self_) -> ::proc_macro::TokenStream {
            crate::expand_or! {
                [$($to)?]
                {
                    let mut ts = Default::default();
                    $self_.to_tokens(&mut ts);
                    ts
                }
            }
        }

        #[cfg(feature = "proc-macro2")]
        fn to_tokens2(&$self_, $tokens: &mut ::proc_macro2::TokenStream) {
            #[allow(unused_imports)]
            use ::proc_macro2 as pm;

            $to_tokens
        }

        #[cfg(feature = "proc-macro2")]
        fn to_token_stream2(&$self_) -> ::proc_macro2::TokenStream {
            crate::expand_or! {
                [$($to)?]
                {
                    let mut ts = Default::default();
                    $self_.to_tokens2(&mut ts);
                    ts
                }
            }
        }
    };
}

macro_rules! impl_into_tokens {
    (tt) => {
        #[cfg(feature = "proc-macro")]
        fn into_tokens(self, tokens: &mut ::proc_macro::TokenStream) {
            tokens.extend(Some(self.into_token_tree()));
        }
        #[cfg(feature = "proc-macro")]
        fn into_token_stream(self) -> ::proc_macro::TokenStream {
            self.into_token_tree().into()
        }
        #[cfg(feature = "proc-macro2")]
        fn into_tokens2(self, tokens: &mut ::proc_macro2::TokenStream) {
            tokens.extend(Some(self.into_token_tree2()));
        }
        #[cfg(feature = "proc-macro2")]
        fn into_token_stream2(self) -> ::proc_macro2::TokenStream {
            self.into_token_tree2().into()
        }

        crate::impl_box_into_tokens! {}
    };
    (#[proxy] |$self_:ident| $proxy:expr) => {
        crate::impl_into_tokens! {
            |$self_, ts| crate::into_st::IntoST::<()>::into_st(($proxy, ts)),
            crate::into_st::IntoST::<_>::into_st($proxy),
        }
    };
    (
        |$self_:ident, $tokens:pat_param $(,)?| $into_tokens:expr
        $(, $into:expr)?
        $(,)?
    ) => {
        #[cfg(feature = "proc-macro")]
        fn into_tokens($self_, $tokens: &mut ::proc_macro::TokenStream) {
            #[allow(unused_imports)]
            use ::proc_macro as pm;
            $into_tokens
        }

        #[cfg(feature = "proc-macro")]
        fn into_token_stream($self_) -> ::proc_macro::TokenStream {
            crate::expand_or! {
                [$(
                    #[allow(unused_imports)]
                    use ::proc_macro as pm;
                    $into
                )?]
                {
                    let mut ts = Default::default();
                    $self_.into_tokens(&mut ts);
                    ts
                }
            }
        }

        #[cfg(feature = "proc-macro2")]
        fn into_tokens2($self_, $tokens: &mut ::proc_macro2::TokenStream) {
            #[allow(unused_imports)]
            use ::proc_macro2 as pm;
            $into_tokens
        }

        #[cfg(feature = "proc-macro2")]
        fn into_token_stream2($self_) -> ::proc_macro2::TokenStream {
            crate::expand_or! {
                [$(
                    #[allow(unused_imports)]
                    use ::proc_macro2 as pm;
                    $into
                )?]
                {
                    let mut ts = Default::default();
                    $self_.into_tokens2(&mut ts);
                    ts
                }
            }
        }

        crate::impl_box_into_tokens! {}
    };
}

macro_rules! impl_box_into_tokens {
    () => {
        #[cfg(feature = "alloc")]
        #[cfg(feature = "proc-macro")]
        fn box_into_tokens(
            self: ::alloc::boxed::Box<Self>,
            tokens: &mut ::proc_macro::TokenStream,
        ) {
            Self::into_tokens(*self, tokens)
        }
        #[cfg(feature = "alloc")]
        #[cfg(feature = "proc-macro")]
        fn box_into_token_stream(self: ::alloc::boxed::Box<Self>) -> ::proc_macro::TokenStream {
            Self::into_token_stream(*self)
        }

        #[cfg(feature = "alloc")]
        #[cfg(feature = "proc-macro2")]
        fn box_into_tokens2(
            self: ::alloc::boxed::Box<Self>,
            tokens: &mut ::proc_macro2::TokenStream,
        ) {
            Self::into_tokens2(*self, tokens)
        }
        #[cfg(feature = "alloc")]
        #[cfg(feature = "proc-macro2")]
        fn box_into_token_stream2(self: ::alloc::boxed::Box<Self>) -> ::proc_macro2::TokenStream {
            Self::into_token_stream2(*self)
        }
    };
}

macro_rules! impl_to_token_tree {
    (copy) => {
        #[cfg(feature = "proc-macro")]
        fn to_token_tree(&self) -> ::proc_macro::TokenTree {
            <Self as crate::IntoTokenTree>::into_token_tree(*self)
        }

        #[cfg(feature = "proc-macro2")]
        fn to_token_tree2(&self) -> ::proc_macro2::TokenTree {
            <Self as crate::IntoTokenTree>::into_token_tree2(*self)
        }
    };
    (
        |$self_:ident| $to:expr
        $(,)?
    ) => {
        #[cfg(feature = "proc-macro")]
        fn to_token_tree(&$self_) -> ::proc_macro::TokenTree {
            #[allow(unused_imports)]
            use ::proc_macro as pm;
            $to
        }

        #[cfg(feature = "proc-macro2")]
        fn to_token_tree2(&$self_) -> ::proc_macro2::TokenTree {
            #[allow(unused_imports)]
            use ::proc_macro2 as pm;
            $to
        }
    };
}

macro_rules! impl_into_token_tree {
    (
        |$self_:ident| $into:expr
        $(,)?
    ) => {
        #[cfg(feature = "proc-macro")]
        fn into_token_tree($self_) -> ::proc_macro::TokenTree {
            #[allow(unused_imports)]
            use ::proc_macro as pm;

            $into
        }

        #[cfg(feature = "proc-macro2")]
        fn into_token_tree2($self_) -> ::proc_macro2::TokenTree {
            #[allow(unused_imports)]
            use ::proc_macro2 as pm;

            $into
        }

        crate::impl_box_into_token_tree! {}
    };
    (to) => {
        crate::impl_into_token_tree! {
            |self| crate::into_st::IntoST::<_>::into_st(&self)
        }
    };
}

macro_rules! impl_box_into_token_tree {
    () => {
        #[cfg(feature = "alloc")]
        #[cfg(feature = "proc-macro")]
        fn box_into_token_tree(self: ::alloc::boxed::Box<Self>) -> ::proc_macro::TokenTree {
            Self::into_token_tree(*self)
        }
        #[cfg(feature = "alloc")]
        #[cfg(feature = "proc-macro2")]
        fn box_into_token_tree2(self: ::alloc::boxed::Box<Self>) -> ::proc_macro2::TokenTree {
            Self::into_token_tree2(*self)
        }
    };
}

macro_rules! impl_ref_with_span {
    (copy) => {
        type RefWithDefaultSpan<'ref_with_span, S: crate::Span>
            = <Self as crate::WithSpan>::WithDefaultSpan<S>
        where
            Self: 'ref_with_span;

        fn ref_with_default_span<S: crate::Span>(
            &self,
            span: S,
        ) -> Self::RefWithDefaultSpan<'_, S> {
            <Self as crate::WithSpan>::with_default_span(*self, span)
        }

        type RefWithReplacedSpan<'ref_with_span, S: crate::Span>
            = <Self as crate::WithSpan>::WithReplacedSpan<S>
        where
            Self: 'ref_with_span;

        fn ref_with_replaced_span<S: crate::Span>(
            &self,
            span: S,
        ) -> Self::RefWithReplacedSpan<'_, S> {
            <Self as crate::WithSpan>::with_replaced_span(*self, span)
        }
    };
}

macro_rules! expand_or {
    ([         ]$($or:tt)*) => [ $($or)* ];
    ([$($e:tt)+]$($or:tt)*) => [ $($e)+  ];
}

use {
    expand_or, impl_box_into_token_tree, impl_box_into_tokens, impl_into_token_tree,
    impl_into_tokens, impl_many, impl_ref_with_span, impl_to_token_tree, impl_to_tokens,
};

#[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
mod into_st;

#[cfg(feature = "alloc")]
mod alloc_imp;

#[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
mod replace_span_of;

#[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
mod proc_macro_n;

#[cfg(test)]
mod tests;
