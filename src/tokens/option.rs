use crate::Either;

use super::*;

impl<T: MaybeSpan> sealed::MaybeSpan for Option<T> {}
impl<T: MaybeSpan> MaybeSpan for Option<T> {
    #[cfg(feature = "proc-macro")]
    fn into_span_or_call_site(self) -> proc_macro::Span {
        self.map_or_else(proc_macro::Span::call_site, T::into_span_or_call_site)
    }
    #[cfg(feature = "proc-macro")]
    fn make_punct(self, punct: proc_macro::Punct) -> proc_macro::Punct {
        if let Some(span) = self {
            span.make_punct(punct)
        } else {
            punct
        }
    }
    #[cfg(feature = "proc-macro")]
    fn make_group(self, g: proc_macro::Group) -> proc_macro::Group {
        if let Some(span) = self {
            span.make_group(g)
        } else {
            g
        }
    }
    #[cfg(feature = "proc-macro")]
    fn make_literal(self, literal: proc_macro::Literal) -> proc_macro::Literal {
        if let Some(span) = self {
            span.make_literal(literal)
        } else {
            literal
        }
    }

    #[cfg(feature = "proc-macro2")]
    fn into_span2_or_call_site(self) -> proc_macro2::Span {
        self.map_or_else(proc_macro2::Span::call_site, T::into_span2_or_call_site)
    }
    #[cfg(feature = "proc-macro2")]
    fn make_punct2(self, punct: proc_macro2::Punct) -> proc_macro2::Punct {
        if let Some(span) = self {
            span.make_punct2(punct)
        } else {
            punct
        }
    }
    #[cfg(feature = "proc-macro2")]
    fn make_group2(self, g: proc_macro2::Group) -> proc_macro2::Group {
        if let Some(span) = self {
            span.make_group2(g)
        } else {
            g
        }
    }
    #[cfg(feature = "proc-macro2")]
    fn make_literal2(self, literal: proc_macro2::Literal) -> proc_macro2::Literal {
        if let Some(span) = self {
            span.make_literal2(literal)
        } else {
            literal
        }
    }

    type Span = T::Span;
    fn try_into_span(self) -> Option<Self::Span> {
        self.and_then(T::try_into_span)
    }

    type WithDefaultSpan<S: crate::Span> = Either<T::Span, S>;
    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        self.try_into_span().map_or(Either::B(span), Either::A)
    }

    type WithReplacedSpan<S: crate::Span> = S;
    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        span
    }
}

impl<T: IntoTokens> sealed::IntoTokens for Option<T> {}
impl<T: IntoTokens> IntoTokens for Option<T> {
    crate::impl_into_tokens!(
        |self, ts| if let Some(t) = self {
            () = (t, ts).into_st()
        },
        self.map(IntoST::into_st).unwrap_or_default(),
    );
}

impl<T: ToTokens> sealed::ToTokens for Option<T> {}
impl<T: ToTokens> ToTokens for Option<T> {
    crate::impl_to_tokens!(
        |self, ts| if let Some(t) = self {
            () = (t, ts).into_st()
        },
        self.as_ref().map(IntoST::into_st).unwrap_or_default(),
    );
}

impl<T: WithSpan> sealed::WithSpan for Option<T> {}
impl<T: WithSpan> WithSpan for Option<T> {
    type WithDefaultSpan<S: crate::Span> = Option<T::WithDefaultSpan<S>>;

    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        self.map(|t| t.with_default_span(span))
    }

    type WithReplacedSpan<S: crate::Span> = Option<T::WithReplacedSpan<S>>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        self.map(|t| t.with_replaced_span(span))
    }
}

impl<T: RefWithSpan> sealed::RefWithSpan for Option<T> {}
impl<T: RefWithSpan> RefWithSpan for Option<T> {
    type RefWithDefaultSpan<'a, S: crate::Span>
        = Option<T::RefWithDefaultSpan<'a, S>>
    where
        Self: 'a;

    fn ref_with_default_span<S: crate::Span>(&self, span: S) -> Self::RefWithDefaultSpan<'_, S> {
        self.as_ref().map(|t| t.ref_with_default_span(span))
    }

    type RefWithReplacedSpan<'a, S: crate::Span>
        = Option<T::RefWithReplacedSpan<'a, S>>
    where
        Self: 'a;

    fn ref_with_replaced_span<S: crate::Span>(&self, span: S) -> Self::RefWithReplacedSpan<'_, S> {
        self.as_ref().map(|t| t.ref_with_replaced_span(span))
    }
}
