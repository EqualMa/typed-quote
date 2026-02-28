use crate::{Either, Span};

use super::*;

macro_rules! either {
    ($v:expr, |$var:pat_param|$out:expr) => {
        match $v {
            Either::A($var) => $out,
            Either::B($var) => $out,
        }
    };
    ($v:expr, $f:expr) => {
        either!($v, |v| $f(v))
    };
}

macro_rules! either_map {
    ($v:expr, |$var:pat_param|$out:expr) => {
        match $v {
            Either::A($var) => Either::A($out),
            Either::B($var) => Either::B($out),
        }
    };
    ($v:expr, $f:expr) => {
        either_map!($v, |v| $f(v))
    };
}

impl<A: MaybeSpan, B: MaybeSpan> sealed::MaybeSpan for Either<A, B> {}
impl<A: MaybeSpan, B: MaybeSpan> MaybeSpan for Either<A, B> {
    #[cfg(feature = "proc-macro")]
    fn into_span_or_call_site(self) -> proc_macro::Span {
        either!(self, MaybeSpan::into_span_or_call_site)
    }
    #[cfg(feature = "proc-macro")]
    fn make_punct(self, punct: proc_macro::Punct) -> proc_macro::Punct {
        either!(self, |v| v.make_punct(punct))
    }
    #[cfg(feature = "proc-macro")]
    fn make_group(self, g: proc_macro::Group) -> proc_macro::Group {
        either!(self, |v| v.make_group(g))
    }

    #[cfg(feature = "proc-macro2")]
    fn into_span2_or_call_site(self) -> proc_macro2::Span {
        either!(self, MaybeSpan::into_span2_or_call_site)
    }
    #[cfg(feature = "proc-macro2")]
    fn make_punct2(self, punct: proc_macro2::Punct) -> proc_macro2::Punct {
        either!(self, |v| v.make_punct2(punct))
    }
    #[cfg(feature = "proc-macro")]
    fn make_group2(self, g: proc_macro2::Group) -> proc_macro2::Group {
        either!(self, |v| v.make_group2(g))
    }

    type Span = Either<A::Span, B::Span>;
    fn try_into_span(self) -> Option<Self::Span> {
        match self {
            Either::A(this) => this.try_into_span().map(Either::A),
            Either::B(this) => this.try_into_span().map(Either::B),
        }
    }

    type WithDefaultSpan<S: Span> = Either<A::WithDefaultSpan<S>, B::WithDefaultSpan<S>>;
    fn with_default_span<S: Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        either_map!(self, |v| v.with_default_span(span))
    }

    type WithReplacedSpan<S: Span> = Either<A::WithReplacedSpan<S>, B::WithReplacedSpan<S>>;
    fn with_replaced_span<S: Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        either_map!(self, |v| v.with_replaced_span(span))
    }
}

#[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
const _: () = {
    use crate::replace_span_of::ReplaceSpanOf;

    impl<A: ReplaceSpanOf<T>, B: ReplaceSpanOf<T>, T: IntoTokens + WithSpan> ReplaceSpanOf<T>
        for Either<A, B>
    {
        type ReplaceSpanOf = Either<A::ReplaceSpanOf, B::ReplaceSpanOf>;

        fn replace_span_of(self, t: T) -> Self::ReplaceSpanOf {
            either_map!(self, |v| v.replace_span_of(t))
        }
    }
};

impl<A: Span, B: Span> sealed::Span for Either<A, B> {}
impl<A: Span, B: Span> Span for Either<A, B> {}

impl<A: IntoTokenTree, B: IntoTokenTree> sealed::IntoTokenTree for Either<A, B> {}
impl<A: IntoTokenTree, B: IntoTokenTree> IntoTokenTree for Either<A, B> {
    crate::impl_into_token_tree!(|self| either!(self, IntoST::into_st));
}

impl<A: ToTokenTree, B: ToTokenTree> sealed::ToTokenTree for Either<A, B> {}
impl<A: ToTokenTree, B: ToTokenTree> ToTokenTree for Either<A, B> {
    crate::impl_to_token_tree!(|self| either!(self, IntoST::into_st));
}

impl<A: IntoTokens, B: IntoTokens> sealed::IntoTokens for Either<A, B> {}
impl<A: IntoTokens, B: IntoTokens> IntoTokens for Either<A, B> {
    crate::impl_into_tokens!(
        |self, ts| either!(self, |v| () = (v, ts).into_st()),
        either!(self, IntoST::into_st),
    );
}

impl<A: ToTokens, B: ToTokens> sealed::ToTokens for Either<A, B> {}
impl<A: ToTokens, B: ToTokens> ToTokens for Either<A, B> {
    crate::impl_to_tokens!(
        |self, ts| either!(self, |v| () = (v, ts).into_st()),
        either!(self, IntoST::into_st),
    );
}

impl<A: WithSpan, B: WithSpan> sealed::WithSpan for Either<A, B> {}
impl<A: WithSpan, B: WithSpan> WithSpan for Either<A, B> {
    type WithDefaultSpan<S: Span> = Either<A::WithDefaultSpan<S>, B::WithDefaultSpan<S>>;

    fn with_default_span<S: Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        either_map!(self, |v| v.with_default_span(span))
    }

    type WithReplacedSpan<S: Span> = Either<A::WithReplacedSpan<S>, B::WithReplacedSpan<S>>;

    fn with_replaced_span<S: Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        either_map!(self, |v| v.with_replaced_span(span))
    }
}

impl<A: RefWithSpan, B: RefWithSpan> sealed::RefWithSpan for Either<A, B> {}
impl<A: RefWithSpan, B: RefWithSpan> RefWithSpan for Either<A, B> {
    type RefWithDefaultSpan<'a, S: Span>
        = Either<A::RefWithDefaultSpan<'a, S>, B::RefWithDefaultSpan<'a, S>>
    where
        Self: 'a;

    fn ref_with_default_span<S: Span>(&self, span: S) -> Self::RefWithDefaultSpan<'_, S> {
        either_map!(self, |v| v.ref_with_default_span(span))
    }

    type RefWithReplacedSpan<'a, S: Span>
        = Either<A::RefWithReplacedSpan<'a, S>, B::RefWithReplacedSpan<'a, S>>
    where
        Self: 'a;

    fn ref_with_replaced_span<S: Span>(&self, span: S) -> Self::RefWithReplacedSpan<'_, S> {
        either_map!(self, |v| v.ref_with_replaced_span(span))
    }
}
