use super::*;

impl<'a, S: MaybeSpan> Ident<'a, S> {
    fn is_raw(&self) -> bool {
        self.0.starts_with("r#")
    }
}

impl<'a, S: MaybeSpan> sealed::IntoTokenTree for Ident<'a, S> {}
impl<'a, S: MaybeSpan> IntoTokenTree for Ident<'a, S> {
    crate::impl_into_token_tree!(|self| pm::TokenTree::Ident(if self.is_raw() {
        pm::Ident::new(self.0, self.1.into_st())
    } else {
        pm::Ident::new_raw(self.0, self.1.into_st())
    }));
}

impl<'a, S: MaybeSpan> sealed::ToTokenTree for Ident<'a, S> {}
impl<'a, S: MaybeSpan> ToTokenTree for Ident<'a, S> {
    crate::impl_to_token_tree! {copy}
}

impl<'a, S: MaybeSpan> sealed::IntoTokens for Ident<'a, S> {}
impl<'a, S: MaybeSpan> IntoTokens for Ident<'a, S> {
    crate::impl_into_tokens! {tt}
}

impl<'a, S: MaybeSpan> sealed::ToTokens for Ident<'a, S> {}
impl<'a, S: MaybeSpan> ToTokens for Ident<'a, S> {
    crate::impl_to_tokens! {tt}
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

impl<'a, S: MaybeSpan> sealed::RefWithSpan for Ident<'a, S> {}
impl<'a, SO: MaybeSpan> RefWithSpan for Ident<'a, SO> {
    crate::impl_ref_with_span!(copy);
}
