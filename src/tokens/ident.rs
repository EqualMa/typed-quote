use super::*;

#[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
impl<'a, S: MaybeSpan> Ident<'a, S> {
    fn to_raw(self) -> (bool, &'a str) {
        match self.0.strip_prefix("r#") {
            Some(ident) => (true, ident),
            None => (false, self.0),
        }
    }
}

impl<'a, S: MaybeSpan> sealed::IntoTokenTree for Ident<'a, S> {}
impl<'a, S: MaybeSpan> IntoTokenTree for Ident<'a, S> {
    crate::impl_into_token_tree!(|self| {
        let (is_raw, ident) = self.to_raw();
        pm::TokenTree::Ident(if is_raw {
            pm::Ident::new_raw(ident, self.1.into_st())
        } else {
            pm::Ident::new(ident, self.1.into_st())
        })
    });
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
