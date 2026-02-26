#[cfg(feature = "proc-macro")]
extern crate proc_macro;

#[cfg(feature = "proc-macro")]
use proc_macro::{TokenStream, TokenTree};

#[cfg(feature = "proc-macro")]
use proc_macro2::{TokenStream as TokenStream2, TokenTree as TokenTree2};

pub trait ToTokens: sealed::ToTokens {
    #[cfg(feature = "proc-macro")]
    fn to_tokens(&self, tokens: &mut TokenStream);
    #[cfg(feature = "proc-macro")]
    fn into_tokens(self, tokens: &mut TokenStream);
    #[cfg(feature = "proc-macro")]
    fn to_token_stream(&self) -> TokenStream;
    #[cfg(feature = "proc-macro")]
    fn into_token_stream(self) -> TokenStream;

    #[cfg(feature = "proc-macro2")]
    fn to_tokens2(&self, tokens: &mut TokenStream2);
    #[cfg(feature = "proc-macro2")]
    fn into_tokens2(self, tokens: &mut TokenStream2);
    #[cfg(feature = "proc-macro2")]
    fn to_token_stream2(&self) -> TokenStream2;
    #[cfg(feature = "proc-macro2")]
    fn into_token_stream2(self) -> TokenStream2;
}

impl<T: ToTokens> sealed::ToTokens for &T {}
impl<T: ToTokens> ToTokens for &T {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        T::to_tokens(self, tokens)
    }

    fn into_tokens(self, tokens: &mut TokenStream) {
        T::to_tokens(self, tokens)
    }

    fn to_token_stream(&self) -> TokenStream {
        T::to_token_stream(self)
    }

    fn into_token_stream(self) -> TokenStream {
        T::to_token_stream(self)
    }

    fn to_tokens2(&self, tokens: &mut TokenStream2) {
        T::to_tokens2(self, tokens)
    }

    fn into_tokens2(self, tokens: &mut TokenStream2) {
        T::to_tokens2(self, tokens)
    }

    fn to_token_stream2(&self) -> TokenStream2 {
        T::to_token_stream2(self)
    }

    fn into_token_stream2(self) -> TokenStream2 {
        T::to_token_stream2(self)
    }
}

pub trait ToTokenTree: ToTokens + sealed::ToTokenTree {
    #[cfg(feature = "proc-macro")]
    fn to_token_tree(&self) -> TokenTree;
    #[cfg(feature = "proc-macro")]
    fn into_token_tree(self) -> TokenTree;

    #[cfg(feature = "proc-macro2")]
    fn to_token_tree2(&self) -> TokenTree2;
    #[cfg(feature = "proc-macro2")]
    fn into_token_tree2(self) -> TokenTree2;
}

impl<T: ToTokenTree> sealed::ToTokenTree for &T {}
impl<T: ToTokenTree> ToTokenTree for &T {
    fn to_token_tree(&self) -> TokenTree {
        T::to_token_tree(self)
    }

    fn into_token_tree(self) -> TokenTree {
        T::to_token_tree(self)
    }

    fn to_token_tree2(&self) -> TokenTree2 {
        T::to_token_tree2(self)
    }

    fn into_token_tree2(self) -> TokenTree2 {
        T::to_token_tree2(self)
    }
}

pub trait Span: sealed::Span + Copy + maybe_span::MaybeSpan {}

pub trait WithSpan: ToTokens + sealed::WithSpan {
    type WithDefaultSpan<S: Span>: ToTokens;

    fn with_default_span<S: Span>(self, span: S) -> Self::WithDefaultSpan<S>;

    type WithReplacedSpan<S: Span>: ToTokens;

    fn with_replaced_span<S: Span>(self, span: S) -> Self::WithReplacedSpan<S>;
}

mod sealed {
    pub trait ToTokens {}
    pub trait ToTokenTree {}
    pub trait MaybeSpan {}
    pub trait Span {}
    pub trait WithSpan {}
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
        $crate::tokens::token::Colon2
    };
    (( $($content:tt)* )) => {
        $crate::tokens::Parenthesis(
            $crate::quote!($($content)*)
        )
    };
    ([ $($content:tt)* ]) => {
        $crate::tokens::Bracket(
            $crate::quote!($($content)*)
        )
    };
    ({ $($content:tt)* }) => {
        $crate::tokens::Brace(
            $crate::quote!($($content)*)
        )
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
        $crate::tokens::punct_group::AddEq
    };
    (&) => {
        $crate::tokens::punct::And
    };
    (&&) => {
        $crate::tokens::punct_group::AndAnd
    };
    (&=) => {
        $crate::tokens::punct_group::AndEq
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
        $crate::tokens::punct_group::CaretEq
    };
    (/) => {
        $crate::tokens::punct::Div
    };
    (/=) => {
        $crate::tokens::punct_group::DivEq
    };
    (..) => {
        $crate::tokens::punct_group::Dot2
    };
    (...) => {
        $crate::tokens::punct_group::Dot3
    };
    (..=) => {
        $crate::tokens::punct_group::DotDotEq
    };
    (=) => {
        $crate::tokens::punct::Eq
    };
    (==) => {
        $crate::tokens::punct_group::EqEq
    };
    (>=) => {
        $crate::tokens::punct_group::Ge
    };
    (>) => {
        $crate::tokens::punct::Gt
    };
    (<=) => {
        $crate::tokens::punct_group::Le
    };
    (<) => {
        $crate::tokens::punct::Lt
    };
    (*=) => {
        $crate::tokens::punct_group::MulEq
    };
    (!=) => {
        $crate::tokens::punct_group::Ne
    };
    (|) => {
        $crate::tokens::punct::Or
    };
    (|=) => {
        $crate::tokens::punct_group::OrEq
    };
    (||) => {
        $crate::tokens::punct_group::OrOr
    };
    (?) => {
        $crate::tokens::punct::Question
    };
    (->) => {
        $crate::tokens::punct_group::RArrow
    };
    (<-) => {
        $crate::tokens::punct_group::LArrow
    };
    (%) => {
        $crate::tokens::punct::Rem
    };
    (%=) => {
        $crate::tokens::punct_group::RemEq
    };
    (=>) => {
        $crate::tokens::punct_group::FatArrow
    };
    (<<) => {
        $crate::tokens::punct_group::Shl
    };
    (<<=) => {
        $crate::tokens::punct_group::ShlEq
    };
    (>>) => {
        $crate::tokens::punct_group::Shr
    };
    (>>=) => {
        $crate::tokens::punct_group::ShrEq
    };
    (*) => {
        $crate::tokens::punct::Star
    };
    (-) => {
        $crate::tokens::punct::Sub
    };
    (-=) => {
        $crate::tokens::punct_group::SubEq
    };
    ($lifetime:lifetime) => {{
        enum Lifetime {}

        impl $crate::tokens::HasConstLifetime {
            const LIFETIME: $crate::tokens::Lifetime =
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
}

macro_rules! impl_to_tokens {
    (
        |$to_tokens_self:ident, $to_tokens_tokens:ident $(,)?| $to_tokens:expr
        $(, into_tokens = $into_tokens:expr)?
        $(, to   =   $to:expr)?
        $(, into = $into:expr)?
        $(,)?
    ) => {
        #[cfg(feature = "proc-macro")]
        fn to_tokens(&$to_tokens_self, $to_tokens_tokens: &mut ::proc_macro::TokenStream) {
            #[allow(unused_imports)]
            use ::proc_macro as pm;

            $to_tokens
        }

        #[cfg(feature = "proc-macro")]
        fn into_tokens($to_tokens_self, $to_tokens_tokens: &mut ::proc_macro::TokenStream) {
            crate::expand_or! {
                [$(
                    #[allow(unused_imports)]
                    use ::proc_macro as pm;
                    $into_tokens
                )?]

                $to_tokens_self.to_tokens($to_tokens_tokens)
            }
        }

        #[cfg(feature = "proc-macro")]
        fn to_token_stream(&$to_tokens_self) -> ::proc_macro::TokenStream {
            crate::expand_or! {
                [$($to)?]
                {
                    let mut ts = Default::default();
                    $to_tokens_self.to_tokens(&mut ts);
                    ts
                }
            }
        }

        #[cfg(feature = "proc-macro")]
        fn into_token_stream($to_tokens_self) -> ::proc_macro::TokenStream {
            crate::expand_or! {
                [$($into)?]
                $to_tokens_self.to_token_stream()
            }
        }

        #[cfg(feature = "proc-macro2")]
        fn to_tokens2(&$to_tokens_self, $to_tokens_tokens: &mut ::proc_macro2::TokenStream) {
            #[allow(unused_imports)]
            use ::proc_macro2 as pm;

            $to_tokens
        }

        #[cfg(feature = "proc-macro2")]
        fn into_tokens2($to_tokens_self, $to_tokens_tokens: &mut ::proc_macro2::TokenStream) {
            crate::expand_or! {
                [$(
                    #[allow(unused_imports)]
                    use ::proc_macro as pm;
                    $into_tokens
                )?]

                $to_tokens_self.to_tokens2($to_tokens_tokens)
            }
        }

        #[cfg(feature = "proc-macro2")]
        fn to_token_stream2(&$to_tokens_self) -> ::proc_macro2::TokenStream {
            crate::expand_or! {
                [$($to)?]
                {
                    let mut ts = Default::default();
                    $to_tokens_self.to_tokens2(&mut ts);
                    ts
                }
            }
        }

        #[cfg(feature = "proc-macro2")]
        fn into_token_stream2($to_tokens_self) -> ::proc_macro2::TokenStream {
            crate::expand_or! {
                [$($into)?]
                $to_tokens_self.to_token_stream2()
            }
        }
    };
}

macro_rules! impl_to_token_tree {
    (
        |$self_:ident| $to:expr
        $(, $into:expr)?
        $(,)?
    ) => {
        #[cfg(feature = "proc-macro")]
        fn to_token_tree(&$self_) -> ::proc_macro::TokenTree {
            #[allow(unused_imports)]
            use ::proc_macro as pm;
            $to
        }

        #[cfg(feature = "proc-macro")]
        fn into_token_tree($self_) -> ::proc_macro::TokenTree {
            crate::expand_or! {[$($into)?] $self_.to_token_tree()}
        }

        #[cfg(feature = "proc-macro2")]
        fn to_token_tree2(&$self_) -> ::proc_macro2::TokenTree {
            #[allow(unused_imports)]
            use ::proc_macro2 as pm;
            $to
        }

        #[cfg(feature = "proc-macro2")]
        fn into_token_tree2($self_) -> ::proc_macro2::TokenTree {
            crate::expand_or! {[$($into)?] $self_.to_token_tree2()}
        }
    };
}

macro_rules! impl_to_tokens_for_tree {
    () => {
        #[cfg(feature = "proc-macro")]
        fn to_tokens(&self, tokens: &mut ::proc_macro::TokenStream) {
            tokens.extend(Some(self.to_token_tree()));
        }
        #[cfg(feature = "proc-macro")]
        fn into_tokens(self, tokens: &mut ::proc_macro::TokenStream) {
            tokens.extend(Some(self.into_token_tree()));
        }
        #[cfg(feature = "proc-macro")]
        fn to_token_stream(&self) -> ::proc_macro::TokenStream {
            self.to_token_tree().into()
        }
        #[cfg(feature = "proc-macro")]
        fn into_token_stream(self) -> ::proc_macro::TokenStream {
            self.into_token_tree().into()
        }

        #[cfg(feature = "proc-macro2")]
        fn to_tokens2(&self, tokens: &mut ::proc_macro2::TokenStream) {
            tokens.extend(Some(self.to_token_tree2()));
        }
        #[cfg(feature = "proc-macro2")]
        fn into_tokens2(self, tokens: &mut ::proc_macro2::TokenStream) {
            tokens.extend(Some(self.into_token_tree2()));
        }
        #[cfg(feature = "proc-macro2")]
        fn to_token_stream2(&self) -> ::proc_macro2::TokenStream {
            self.to_token_tree2().into()
        }
        #[cfg(feature = "proc-macro2")]
        fn into_token_stream2(self) -> ::proc_macro2::TokenStream {
            self.into_token_tree2().into()
        }
    };
}

macro_rules! expand_or {
    ([         ]$($or:tt)*) => [ $($or)* ];
    ([$($e:tt)+]$($or:tt)*) => [ $($e)+  ];
}

use {expand_or, impl_to_token_tree, impl_to_tokens, impl_to_tokens_for_tree};

#[cfg(todo)]
mod proc_macro_1;

mod into_st;

#[cfg(test)]
mod tests;
