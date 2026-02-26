use std::marker::PhantomData;

use crate::{
    ToTokenTree, ToTokens, WithSpan,
    into_st::IntoST as _,
    maybe_span::{MaybeSpan, NoSpan},
    sealed,
};

// region: Empty
pub struct Empty;

impl sealed::ToTokens for Empty {}
impl ToTokens for Empty {
    crate::impl_to_tokens!(
        |self, _ts| {},
        to = Default::default(),
        into = Default::default(),
    );
}
impl sealed::WithSpan for Empty {}
impl WithSpan for Empty {
    type WithDefaultSpan<S: crate::Span> = Self;

    fn with_default_span<S: crate::Span>(self, _: S) -> Self::WithDefaultSpan<S> {
        self
    }

    type WithReplacedSpan<S: crate::Span> = Self;

    fn with_replaced_span<S: crate::Span>(self, _: S) -> Self::WithReplacedSpan<S> {
        self
    }
}

// endregion
// region: group

pub struct Parenthesis<Inner>(pub Inner);
pub struct ParenthesisWithSpan<Inner, Span: crate::Span>(pub Inner, pub Span);
pub struct Bracket<Inner>(pub Inner);
pub struct BracketWithSpan<Inner, Span: crate::Span>(pub Inner, pub Span);
pub struct Brace<Inner>(pub Inner);
pub struct BraceWithSpan<Inner, Span: crate::Span>(pub Inner, pub Span);

// endregion
// region: Ident & Lifetime
pub struct Ident<'a, S: MaybeSpan = NoSpan>(&'a str, S);

impl<'a, S: MaybeSpan> Ident<'a, S> {
    fn is_raw(&self) -> bool {
        self.0.starts_with("r#")
    }
}

impl<'a, S: MaybeSpan> sealed::ToTokenTree for Ident<'a, S> {}
impl<'a, S: MaybeSpan> ToTokenTree for Ident<'a, S> {
    crate::impl_to_token_tree!(|self| {
        if self.is_raw() {
            pm::Ident::new(self.0, self.1.into_st())
        } else {
            pm::Ident::new_raw(self.0, self.1.into_st())
        }
        .into()
    });
}
impl<'a, S: MaybeSpan> sealed::ToTokens for Ident<'a, S> {}
impl<'a, S: MaybeSpan> ToTokens for Ident<'a, S> {
    crate::impl_to_tokens_for_tree! {}
}
impl<'a, S: MaybeSpan> sealed::WithSpan for Ident<'a, S> {}
impl<'a, SO: MaybeSpan> WithSpan for Ident<'a, SO> {
    type WithDefaultSpan<S: crate::Span> = Ident<'a, SO::WithDefaultSpan<S>>;
    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        Ident(self.0, self.1.with_default_span(span))
    }

    type WithReplacedSpan<S: crate::Span> = Ident<'a, SO::WithReplacedSpan<S>>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        Ident(self.0, self.1.with_replaced_span(span))
    }
}

/// A leading `'` is included.
pub struct Lifetime<'a, S: MaybeSpan = NoSpan>(&'a str, S);

impl<'a, S: MaybeSpan> Lifetime<'a, S> {
    fn ident(&self) -> Ident<'a, S> {
        Ident(&self.0[1..], self.1)
    }
}

impl<'a, S: MaybeSpan> sealed::ToTokens for Lifetime<'a, S> {}
impl<'a, S: MaybeSpan> ToTokens for Lifetime<'a, S> {
    crate::impl_to_tokens!(|self, ts| {
        let punct = pm::Punct::new('\'', pm::Spacing::Joint);
        ts.extend([
            pm::TokenTree::from((punct, self.1).into_st()),
            self.ident().into_st(),
        ]);
    });
}

pub trait HasConstIdent {
    const IDENT: Ident<'static>;
}

pub struct ConstIdent<T: HasConstIdent + ?Sized>(PhantomData<T>);

impl<T: HasConstIdent + ?Sized> sealed::ToTokenTree for ConstIdent<T> {}
impl<T: HasConstIdent + ?Sized> ToTokenTree for ConstIdent<T> {
    crate::impl_to_token_tree!(|self| T::IDENT.into_st());
}
impl<T: HasConstIdent + ?Sized> sealed::ToTokens for ConstIdent<T> {}
impl<T: HasConstIdent + ?Sized> ToTokens for ConstIdent<T> {
    crate::impl_to_tokens_for_tree! {}
}

pub trait HasConstLifetime {
    const LIFETIME: Lifetime<'static>;
}

pub struct ConstLifetime<T: HasConstLifetime + ?Sized>(PhantomData<T>);

impl<T: HasConstLifetime + ?Sized> sealed::ToTokens for ConstLifetime<T> {}
impl<T: HasConstLifetime + ?Sized> ToTokens for ConstLifetime<T> {
    crate::impl_to_tokens!(|self, ts| (&T::LIFETIME, ts).into_st());
}

// endregion
// region: Concat
pub struct Concat<A: ToTokens, B: ToTokens>(pub A, pub B);
pub struct ConcatWithDefaultSpan<A: ToTokens, B: ToTokens, S: crate::Span>(pub A, pub B, pub S);
pub struct ConcatWithReplacedSpan<A: ToTokens, B: ToTokens, S: crate::Span>(pub A, pub B, pub S);

impl<A: ToTokens, B: ToTokens> sealed::ToTokens for Concat<A, B> {}
impl<A: ToTokens, B: ToTokens> ToTokens for Concat<A, B> {
    crate::impl_to_tokens!(
        |self, ts| {
            () = (&self.0, &mut *ts).into_st();
            () = (&self.1, ts).into_st();
        },
        into_tokens = {
            () = (self.0, &mut *ts).into_st();
            () = (self.1, ts).into_st();
        },
        to = {
            let mut ts = (&self.0).into_st();
            () = (&self.1, &mut ts).into_st();
            ts
        },
        into = {
            let mut ts = self.0.into_st();
            () = (self.1, &mut ts).into_st();
            ts
        },
    );
}
impl<A: WithSpan, B: WithSpan> sealed::WithSpan for Concat<A, B> {}
impl<A: WithSpan, B: WithSpan> WithSpan for Concat<A, B> {
    type WithDefaultSpan<S: crate::Span> = ConcatWithDefaultSpan<A, B, S>;

    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        ConcatWithDefaultSpan(self.0, self.1, span)
    }

    type WithReplacedSpan<S: crate::Span> = ConcatWithReplacedSpan<A, B, S>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        ConcatWithReplacedSpan(self.0, self.1, span)
    }
}

impl<A: ToTokens, B: ToTokens, S: crate::Span> sealed::ToTokens for ConcatWithDefaultSpan<A, B, S> {}
impl<A: ToTokens, B: ToTokens, S: crate::Span> ToTokens for ConcatWithDefaultSpan<A, B, S> {
    crate::impl_to_tokens!(
        |self, ts| {
            () = (&self.0, &mut *ts).into_st();
            () = (&self.1, ts).into_st();
        },
        into_tokens = {
            () = (self.0, &mut *ts).into_st();
            () = (self.1, ts).into_st();
        },
        to = {
            let mut ts = (&self.0).into_st();
            () = (&self.1, &mut ts).into_st();
            ts
        },
        into = {
            let mut ts = self.0.into_st();
            () = (self.1, &mut ts).into_st();
            ts
        },
    );
}
impl<A: WithSpan, B: WithSpan> sealed::WithSpan for Concat<A, B> {}
impl<A: WithSpan, B: WithSpan> WithSpan for Concat<A, B> {
    type WithDefaultSpan<S: crate::Span> = ConcatWithDefaultSpan<A, B, S>;

    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        ConcatWithDefaultSpan(self.0, self.1, span)
    }

    type WithReplacedSpan<S: crate::Span> = ConcatWithReplacedSpan<A, B, S>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        ConcatWithReplacedSpan(self.0, self.1, span)
    }
}

// endregion
// region: iter
pub struct IterTokenStream<I>(pub I);
pub struct IterTokenStreamWithDefaultSpan<I, S>(pub I, pub S);
pub struct IterTokenStreamWithReplacedSpan<I, S>(pub I, pub S);

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
    pub const fn ident(ident: &'static str) -> Ident {
        Ident(ident, super::NoSpan)
    }
}
