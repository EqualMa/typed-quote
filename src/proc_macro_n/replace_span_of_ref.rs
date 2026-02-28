use super::*;

#[derive(Debug, Clone)]
pub struct ReplaceSpanOfRef<'a, T: Clone + IntoTokens + WithSpan, S: Span>(
    pub(super) &'a T,
    pub(super) S,
);

impl<'a, T: Clone + IntoTokens + WithSpan, S: Span> Copy for ReplaceSpanOfRef<'a, T, S> {}

impl<'a, T: Clone + IntoTokens + WithSpan, S: Span> ReplaceSpanOfRef<'a, T, S> {
    fn cloned(self) -> T::WithReplacedSpan<S> {
        T::clone(self.0).with_replaced_span(self.1)
    }
}

impl<'a, T: Clone + IntoTokenTree + WithSpan<WithReplacedSpan<S>: IntoTokenTree>, S: Span>
    sealed::IntoTokenTree for ReplaceSpanOfRef<'a, T, S>
{
}
impl<'a, T: Clone + IntoTokenTree + WithSpan<WithReplacedSpan<S>: IntoTokenTree>, S: Span>
    IntoTokenTree for ReplaceSpanOfRef<'a, T, S>
{
    crate::impl_into_token_tree!(|self| self.cloned().into_st());
}

impl<'a, T: Clone + ToTokenTree + WithSpan<WithReplacedSpan<S>: IntoTokenTree>, S: Span>
    sealed::ToTokenTree for ReplaceSpanOfRef<'a, T, S>
{
}
impl<'a, T: Clone + ToTokenTree + WithSpan<WithReplacedSpan<S>: IntoTokenTree>, S: Span> ToTokenTree
    for ReplaceSpanOfRef<'a, T, S>
{
    crate::impl_to_token_tree! {copy}
}

impl<'a, T: Clone + IntoTokens + WithSpan, S: Span> sealed::IntoTokens
    for ReplaceSpanOfRef<'a, T, S>
{
}
impl<'a, T: Clone + IntoTokens + WithSpan, S: Span> IntoTokens for ReplaceSpanOfRef<'a, T, S> {
    crate::impl_into_tokens!(
        |self, ts| (self.cloned(), ts).into_st(),
        self.cloned().into_st(),
    );
}

impl<'a, T: Clone + IntoTokens + WithSpan, S: Span> sealed::ToTokens
    for ReplaceSpanOfRef<'a, T, S>
{
}
impl<'a, T: Clone + IntoTokens + WithSpan, S: Span> ToTokens for ReplaceSpanOfRef<'a, T, S> {
    crate::impl_to_tokens! {copy}
}

impl<'a, T: Clone + IntoTokens + WithSpan, SO: Span> sealed::WithSpan
    for ReplaceSpanOfRef<'a, T, SO>
{
}
impl<'a, T: Clone + IntoTokens + WithSpan, SO: Span> WithSpan for ReplaceSpanOfRef<'a, T, SO> {
    type WithDefaultSpan<S: Span> = Self;

    fn with_default_span<S: Span>(self, _: S) -> Self::WithDefaultSpan<S> {
        self
    }

    type WithReplacedSpan<S: Span> = ReplaceSpanOfRef<'a, T, S>;

    fn with_replaced_span<S: Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        ReplaceSpanOfRef(self.0, span)
    }
}

impl<'a, T: Clone + IntoTokens + WithSpan, SO: Span> sealed::RefWithSpan
    for ReplaceSpanOfRef<'a, T, SO>
{
}
impl<'this, T: Clone + IntoTokens + WithSpan, SO: Span> RefWithSpan
    for ReplaceSpanOfRef<'this, T, SO>
{
    crate::impl_ref_with_span! {copy}
}
