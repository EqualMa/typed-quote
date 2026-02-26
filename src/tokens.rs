use std::marker::PhantomData;

use crate::{
    IntoTokenTree, IntoTokens, RefWithSpan, ToTokenTree, ToTokens, WithSpan,
    into_st::IntoST as _,
    maybe_span::{MaybeSpan, NoSpan},
    sealed,
};

#[derive(Debug, Clone, Copy)]
pub struct Empty;
mod empty;

// region: group

pub struct Parenthesis<Inner>(pub Inner);
pub struct ParenthesisWithSpan<Inner, Span: crate::Span>(pub Inner, pub Span);
pub struct Bracket<Inner>(pub Inner);
pub struct BracketWithSpan<Inner, Span: crate::Span>(pub Inner, pub Span);
pub struct Brace<Inner>(pub Inner);
pub struct BraceWithSpan<Inner, Span: crate::Span>(pub Inner, pub Span);

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
// https://doc.rust-lang.org/stable/src/proc_macro/lib.rs.html#959
#[cfg(todo)]
pub mod punct {

    mod pm1 {
        pub use proc_macro::{Span, TokenTree};
    }

    macro_rules! punct {
        (
            #$attr:tt
            type $This:ident = each_of![
                $($PUNCT_value:literal as $Punct:ident),+ $(,)?
            ];
            const _: () = $impl_body:tt ;
        ) => {
            $(
                #$attr
                pub struct $Punct;
            )+
            $(
                const _: () = {
                    type $This = $Punct;
                    const $This : char = $PUNCT_value;

                    $impl_body
                };
            )+
        };
    }

    punct!(
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        type PUNCT = each_of![
            //
            '=' as Eq,
            ',' as Comma,
            ';' as Semi,
            ':' as Colon,
            '!' as Bang,
        ];
        const _: () = {
            fn punct() -> proc_macro::Punct {
                proc_macro::Punct::new(PUNCT, proc_macro::Spacing::Alone)
            }

            impl pm1::WithSpan for PUNCT {
                type WithDefaultSpan = Self::WithReplacedSpan;

                fn with_default_span(self, span: pm1::Span) -> Self::WithDefaultSpan {
                    self.with_replaced_span(span)
                }

                type WithReplacedSpan = pm1::WithReplacedSpan<Self>;

                fn with_replaced_span(self, span: pm1::Span) -> Self::WithReplacedSpan {
                    pm1::WithReplacedSpan(self, span)
                }
            }

            impl pm1::IntoTokenTree for PUNCT {
                fn into_token_tree(self) -> pm1::TokenTree {
                    pm1::TokenTree::Punct(punct())
                }
            }

            impl pm1::IntoTokenTree for pm1::WithReplacedSpan<PUNCT> {
                fn into_token_tree(self) -> pm1::TokenTree {
                    let Self(_, span) = self;
                    let mut tt = punct();
                    tt.set_span(span);
                    pm1::TokenTree::Punct(tt)
                }
            }
        };
    );
}

pub mod __private {
    pub use core::stringify;

    use super::Ident;

    /// `ident` must be `stringify!($ident)` where `$ident:ident`
    pub const fn ident(ident: &'static str) -> Ident<'static> {
        Ident(ident, super::NoSpan)
    }
}
