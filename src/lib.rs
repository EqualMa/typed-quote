#[cfg(feature = "proc-macro")]
extern crate proc_macro;

#[cfg(feature = "proc-macro")]
use proc_macro::{TokenStream, TokenTree};

#[cfg(feature = "proc-macro")]
use proc_macro2::{TokenStream as TokenStream2, TokenTree as TokenTree2};

pub trait IntoTokens: sealed::IntoTokens {
    #[cfg(feature = "proc-macro")]
    fn into_tokens(self, tokens: &mut TokenStream);
    #[cfg(feature = "proc-macro")]
    fn into_token_stream(self) -> TokenStream;

    #[cfg(feature = "proc-macro2")]
    fn into_tokens2(self, tokens: &mut TokenStream2);
    #[cfg(feature = "proc-macro2")]
    fn into_token_stream2(self) -> TokenStream2;
}

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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        T::to_tokens(self, tokens)
    }

    fn to_token_stream(&self) -> TokenStream {
        T::to_token_stream(self)
    }

    fn to_tokens2(&self, tokens: &mut TokenStream2) {
        T::to_tokens2(self, tokens)
    }

    fn to_token_stream2(&self) -> TokenStream2 {
        T::to_token_stream2(self)
    }
}
impl<T: ?Sized + ToTokens> sealed::IntoTokens for &T {}
impl<T: ?Sized + ToTokens> IntoTokens for &T {
    fn into_tokens(self, tokens: &mut TokenStream) {
        T::to_tokens(self, tokens)
    }
    fn into_token_stream(self) -> TokenStream {
        T::to_token_stream(self)
    }
    fn into_tokens2(self, tokens: &mut TokenStream2) {
        T::to_tokens2(self, tokens)
    }
    fn into_token_stream2(self) -> TokenStream2 {
        T::to_token_stream2(self)
    }
}

pub trait IntoTokenTree: IntoTokens + sealed::IntoTokenTree {
    #[cfg(feature = "proc-macro")]
    fn into_token_tree(self) -> TokenTree;
    #[cfg(feature = "proc-macro2")]
    fn into_token_tree2(self) -> TokenTree2;
}

pub trait ToTokenTree: ToTokens + sealed::ToTokenTree {
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
}

pub trait Span: sealed::Span + Copy + maybe_span::MaybeSpan {}

pub trait WithSpan: IntoTokens + sealed::WithSpan {
    type WithDefaultSpan<S: Span>: IntoTokens + WithSpan;

    fn with_default_span<S: Span>(self, span: S) -> Self::WithDefaultSpan<S>;

    type WithReplacedSpan<S: Span>: IntoTokens + WithSpan;

    fn with_replaced_span<S: Span>(self, span: S) -> Self::WithReplacedSpan<S>;
}

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

impl<'a, T: ?Sized + RefWithSpan> sealed::RefWithSpan for &'a T {}
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

impl<'a, T: ?Sized + RefWithSpan> sealed::WithSpan for &'a T {}
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
    pub trait IntoTokens {}
    pub trait ToTokens {}

    pub trait IntoTokenTree {}
    pub trait ToTokenTree {}

    pub trait MaybeSpan {}
    pub trait Span {}

    pub trait WithSpan {}
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
    ($ident:ident) => {{
        enum Ident {}
        impl $crate::tokens::HasConstIdent for Ident {
            const IDENT: $crate::tokens::Ident<'static> =
                $crate::tokens::__private::ident($crate::tokens::__private::stringify!($ident));
        }
        $crate::tokens::ConstIdent::<Ident>::new()
    }};
    (::) => {
        $crate::tokens::puncts::Colon2
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
    (#) => {
        $crate::tokens::punct::Pound
    };
    (,) => {
        $crate::tokens::punct::Comma
    };
    (.) => {
        $crate::tokens::punct::Dot
    };
    (;) => {
        $crate::tokens::punct::Semi
    };
    (:) => {
        $crate::tokens::punct::Colon
    };
    (+) => {
        $crate::tokens::punct::Add
    };
    (+=) => {
        $crate::tokens::puncts::AddEq
    };
    (&) => {
        $crate::tokens::punct::And
    };
    (&&) => {
        $crate::tokens::puncts::AndAnd
    };
    (&=) => {
        $crate::tokens::puncts::AndEq
    };
    (@) => {
        $crate::tokens::punct::At
    };
    (!) => {
        $crate::tokens::punct::Bang
    };
    (^) => {
        $crate::tokens::punct::Caret
    };
    (^=) => {
        $crate::tokens::puncts::CaretEq
    };
    (/) => {
        $crate::tokens::punct::Div
    };
    (/=) => {
        $crate::tokens::puncts::DivEq
    };
    (..) => {
        $crate::tokens::puncts::Dot2
    };
    (...) => {
        $crate::tokens::puncts::Dot3
    };
    (..=) => {
        $crate::tokens::puncts::DotDotEq
    };
    (=) => {
        $crate::tokens::punct::Eq
    };
    (==) => {
        $crate::tokens::puncts::EqEq
    };
    (>=) => {
        $crate::tokens::puncts::Ge
    };
    (>) => {
        $crate::tokens::punct::Gt
    };
    (<=) => {
        $crate::tokens::puncts::Le
    };
    (<) => {
        $crate::tokens::punct::Lt
    };
    (*=) => {
        $crate::tokens::puncts::MulEq
    };
    (!=) => {
        $crate::tokens::puncts::Ne
    };
    (|) => {
        $crate::tokens::punct::Or
    };
    (|=) => {
        $crate::tokens::puncts::OrEq
    };
    (||) => {
        $crate::tokens::puncts::OrOr
    };
    (?) => {
        $crate::tokens::punct::Question
    };
    (->) => {
        $crate::tokens::puncts::RArrow
    };
    (<-) => {
        $crate::tokens::puncts::LArrow
    };
    (%) => {
        $crate::tokens::punct::Rem
    };
    (%=) => {
        $crate::tokens::puncts::RemEq
    };
    (=>) => {
        $crate::tokens::puncts::FatArrow
    };
    (<<) => {
        $crate::tokens::puncts::Shl
    };
    (<<=) => {
        $crate::tokens::puncts::ShlEq
    };
    (>>) => {
        $crate::tokens::puncts::Shr
    };
    (>>=) => {
        $crate::tokens::puncts::ShrEq
    };
    (*) => {
        $crate::tokens::punct::Star
    };
    (-) => {
        $crate::tokens::punct::Sub
    };
    (-=) => {
        $crate::tokens::puncts::SubEq
    };
    ($lifetime:lifetime) => {{
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
}

pub mod maybe_span;

pub mod tokens;

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
    };
    (to) => {
        crate::impl_into_token_tree! {
            |self| crate::into_st::IntoST::<_>::into_st(&self)
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
    expand_or, impl_into_token_tree, impl_into_tokens, impl_many, impl_ref_with_span,
    impl_to_token_tree, impl_to_tokens,
};

#[cfg(todo)]
mod proc_macro_1;

mod into_st;

#[cfg(test)]
mod tests;
