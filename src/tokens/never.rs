use crate::{Never, Span};

use super::*;

impl sealed::MaybeSpan for Never {}
impl MaybeSpan for Never {
    #[cfg(feature = "proc-macro")]
    fn into_span_or_call_site(self) -> proc_macro::Span {
        match self {}
    }
    #[cfg(feature = "proc-macro")]
    fn make_punct(self, _: proc_macro::Punct) -> proc_macro::Punct {
        match self {}
    }
    #[cfg(feature = "proc-macro")]
    fn make_group(self, _: proc_macro::Group) -> proc_macro::Group {
        match self {}
    }
    #[cfg(feature = "proc-macro")]
    fn make_literal(self, _: proc_macro::Literal) -> proc_macro::Literal {
        match self {}
    }

    #[cfg(feature = "proc-macro2")]
    fn into_span2_or_call_site(self) -> proc_macro2::Span {
        match self {}
    }
    #[cfg(feature = "proc-macro2")]
    fn make_punct2(self, _: proc_macro2::Punct) -> proc_macro2::Punct {
        match self {}
    }
    #[cfg(feature = "proc-macro2")]
    fn make_group2(self, _: proc_macro2::Group) -> proc_macro2::Group {
        match self {}
    }
    #[cfg(feature = "proc-macro2")]
    fn make_literal2(self, _: proc_macro2::Literal) -> proc_macro2::Literal {
        match self {}
    }

    type Span = Self;
    fn try_into_span(self) -> Option<Self::Span> {
        match self {}
    }

    type WithDefaultSpan<S: Span> = Self;
    fn with_default_span<S: Span>(self, _: S) -> Self::WithDefaultSpan<S> {
        match self {}
    }

    type WithReplacedSpan<S: Span> = Self;
    fn with_replaced_span<S: Span>(self, _: S) -> Self::WithReplacedSpan<S> {
        match self {}
    }
}

#[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
impl<T: IntoTokens + WithSpan> crate::replace_span_of::ReplaceSpanOf<T> for Never {
    type ReplaceSpanOf = Never;

    fn replace_span_of(self, _: T) -> Self::ReplaceSpanOf {
        match self {}
    }
}

impl sealed::Span for Never {}
impl Span for Never {}

impl sealed::IntoTokenTree for Never {}
impl IntoTokenTree for Never {
    crate::impl_into_token_tree!(|self| match self {});
}

impl sealed::ToTokenTree for Never {}
impl ToTokenTree for Never {
    crate::impl_to_token_tree!(|self| match *self {});
}

impl sealed::ToTokens for Never {}
impl ToTokens for Never {
    crate::impl_to_tokens!(|self, _| match *self {}, match *self {});
}

impl sealed::IntoTokens for Never {}
impl IntoTokens for Never {
    crate::impl_into_tokens!(|self, _| match self {}, match self {});
}

impl sealed::WithSpan for Never {}
impl WithSpan for Never {
    type WithDefaultSpan<S: Span> = Never;

    fn with_default_span<S: Span>(self, _: S) -> Self::WithDefaultSpan<S> {
        match self {}
    }

    type WithReplacedSpan<S: Span> = Never;

    fn with_replaced_span<S: Span>(self, _: S) -> Self::WithReplacedSpan<S> {
        match self {}
    }
}

impl sealed::RefWithSpan for Never {}
impl RefWithSpan for Never {
    type RefWithDefaultSpan<'a, S: Span>
        = Never
    where
        Self: 'a;

    fn ref_with_default_span<S: Span>(&self, _: S) -> Self::RefWithDefaultSpan<'_, S> {
        match *self {}
    }

    type RefWithReplacedSpan<'a, S: Span>
        = Never
    where
        Self: 'a;

    fn ref_with_replaced_span<S: Span>(&self, _: S) -> Self::RefWithReplacedSpan<'_, S> {
        match *self {}
    }
}
