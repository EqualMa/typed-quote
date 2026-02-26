use super::*;

impl<'a, S: MaybeSpan> Lifetime<'a, S> {
    const fn ident(&self) -> Ident<'a, S> {
        Ident(self.0.split_at(1).1, self.1)
    }
}

impl<'a, S: MaybeSpan> sealed::IntoTokens for Lifetime<'a, S> {}
impl<'a, S: MaybeSpan> IntoTokens for Lifetime<'a, S> {
    crate::impl_into_tokens!(|self, ts| {
        let punct = pm::Punct::new('\'', pm::Spacing::Joint);
        ts.extend([
            pm::TokenTree::from((punct, self.1).into_st()),
            self.ident().into_st(),
        ]);
    });
}

impl<'a, S: MaybeSpan> sealed::ToTokens for Lifetime<'a, S> {}
impl<'a, S: MaybeSpan> ToTokens for Lifetime<'a, S> {
    crate::impl_to_tokens! {copy}
}

impl<'a, S: MaybeSpan> sealed::WithSpan for Lifetime<'a, S> {}
impl<'a, SO: MaybeSpan> WithSpan for Lifetime<'a, SO> {
    type WithDefaultSpan<S: crate::Span> = Lifetime<'a, SO::WithDefaultSpan<S>>;

    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        Lifetime(self.0, self.1.with_default_span(span))
    }

    type WithReplacedSpan<S: crate::Span> = Lifetime<'a, SO::WithReplacedSpan<S>>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        Lifetime(self.0, self.1.with_replaced_span(span))
    }
}

impl<'a, S: MaybeSpan> sealed::RefWithSpan for Lifetime<'a, S> {}
impl<'a, SO: MaybeSpan> RefWithSpan for Lifetime<'a, SO> {
    crate::impl_ref_with_span! {copy}
}
