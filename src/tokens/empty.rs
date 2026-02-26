use super::*;

impl sealed::IntoTokens for Empty {}
impl IntoTokens for Empty {
    crate::impl_into_tokens!(|self, _| {}, Default::default(),);
}

impl sealed::ToTokens for Empty {}
impl ToTokens for Empty {
    crate::impl_to_tokens! {copy}
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

impl sealed::RefWithSpan for Empty {}
impl RefWithSpan for Empty {
    crate::impl_ref_with_span! {copy}
}
