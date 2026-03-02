use core::str::FromStr;

use super::*;

impl<'a, S: MaybeSpan> sealed::IntoTokenTree for Literal<'a, S> {}
impl<'a, S: MaybeSpan> IntoTokenTree for Literal<'a, S> {
    crate::impl_into_token_tree!(|self| pm::TokenTree::Literal(
        (
            pm::Literal::from_str(self.0).expect("should be a stringified literal"),
            self.1
        )
            .into_st()
    ));
}

impl<'a, S: MaybeSpan> sealed::ToTokenTree for Literal<'a, S> {}
impl<'a, S: MaybeSpan> ToTokenTree for Literal<'a, S> {
    crate::impl_to_token_tree! {copy}
}

impl<'a, S: MaybeSpan> sealed::IntoTokens for Literal<'a, S> {}
impl<'a, S: MaybeSpan> IntoTokens for Literal<'a, S> {
    crate::impl_into_tokens! {tt}
}

impl<'a, S: MaybeSpan> sealed::ToTokens for Literal<'a, S> {}
impl<'a, S: MaybeSpan> ToTokens for Literal<'a, S> {
    crate::impl_to_tokens! {tt}
}
impl<'a, S: MaybeSpan> sealed::WithSpan for Literal<'a, S> {}
impl<'a, SO: MaybeSpan> WithSpan for Literal<'a, SO> {
    type WithDefaultSpan<S: crate::Span> = Literal<'a, SO::WithDefaultSpan<S>>;
    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        Literal(self.0, self.1.with_default_span(span))
    }

    type WithReplacedSpan<S: crate::Span> = Literal<'a, SO::WithReplacedSpan<S>>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        Literal(self.0, self.1.with_replaced_span(span))
    }
}

impl<'a, S: MaybeSpan> sealed::RefWithSpan for Literal<'a, S> {}
impl<'a, SO: MaybeSpan> RefWithSpan for Literal<'a, SO> {
    crate::impl_ref_with_span!(copy);
}
