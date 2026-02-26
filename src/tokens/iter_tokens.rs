use super::*;

impl<I: IntoIterator<Item: IntoTokens>> sealed::IntoTokens for IterTokens<I> {}
impl<I: IntoIterator<Item: IntoTokens>> IntoTokens for IterTokens<I> {
    crate::impl_into_tokens!(|self, ts| {
        self.0.into_iter().for_each(|s| (s, &mut *ts).into_st());
    });
}

impl<I: IntoIterator<Item: IntoTokens>, S: crate::Span> sealed::IntoTokens
    for IterTokensWithDefaultSpan<I, S>
{
}
impl<I: IntoIterator<Item: IntoTokens + WithSpan>, S: crate::Span> IntoTokens
    for IterTokensWithDefaultSpan<I, S>
{
    crate::impl_into_tokens!(
        #[proxy]
        |self| IterTokens(self.0.into_iter().map(|s| s.with_default_span(self.1)))
    );
}

impl<I: IntoIterator<Item: IntoTokens + WithSpan>, S: crate::Span> sealed::IntoTokens
    for IterTokensWithReplacedSpan<I, S>
{
}
impl<I: IntoIterator<Item: IntoTokens + WithSpan>, S: crate::Span> IntoTokens
    for IterTokensWithReplacedSpan<I, S>
{
    crate::impl_into_tokens!(
        #[proxy]
        |self| IterTokens(self.0.into_iter().map(|s| s.with_replaced_span(self.1)))
    );
}

impl<I: IntoIterator<Item: IntoTokens> + Clone> sealed::ToTokens for IterTokens<I> {}
impl<I: IntoIterator<Item: IntoTokens> + Clone> ToTokens for IterTokens<I> {
    crate::impl_to_tokens!(
        #[proxy]
        |self| Self::clone(self)
    );
}

impl<I: IntoIterator<Item: IntoTokens + WithSpan> + Clone, S: crate::Span> sealed::ToTokens
    for IterTokensWithDefaultSpan<I, S>
{
}
impl<I: IntoIterator<Item: IntoTokens + WithSpan> + Clone, S: crate::Span> ToTokens
    for IterTokensWithDefaultSpan<I, S>
{
    crate::impl_to_tokens!(
        #[proxy]
        |self| Self::clone(self)
    );
}

impl<I: IntoIterator<Item: IntoTokens + WithSpan> + Clone, S: crate::Span> sealed::ToTokens
    for IterTokensWithReplacedSpan<I, S>
{
}
impl<I: IntoIterator<Item: IntoTokens + WithSpan> + Clone, S: crate::Span> ToTokens
    for IterTokensWithReplacedSpan<I, S>
{
    crate::impl_to_tokens!(
        #[proxy]
        |self| Self::clone(self)
    );
}

impl<I: IntoIterator<Item: IntoTokens + WithSpan>> sealed::WithSpan for IterTokens<I> {}
impl<I: IntoIterator<Item: IntoTokens + WithSpan>> WithSpan for IterTokens<I> {
    type WithDefaultSpan<S: crate::Span> = IterTokensWithDefaultSpan<I, S>;

    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        IterTokensWithDefaultSpan(self.0, span)
    }

    type WithReplacedSpan<S: crate::Span> = IterTokensWithReplacedSpan<I, S>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        IterTokensWithReplacedSpan(self.0, span)
    }
}

impl<I: IntoIterator<Item: IntoTokens + WithSpan>, SO: crate::Span> sealed::WithSpan
    for IterTokensWithDefaultSpan<I, SO>
{
}
impl<I: IntoIterator<Item: IntoTokens + WithSpan>, SO: crate::Span> WithSpan
    for IterTokensWithDefaultSpan<I, SO>
{
    type WithDefaultSpan<S: crate::Span> = Self;

    fn with_default_span<S: crate::Span>(self, _: S) -> Self::WithDefaultSpan<S> {
        self
    }

    type WithReplacedSpan<S: crate::Span> = IterTokensWithReplacedSpan<I, S>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        IterTokensWithReplacedSpan(self.0, span)
    }
}

impl<I: IntoIterator<Item: IntoTokens + WithSpan>, SO: crate::Span> sealed::WithSpan
    for IterTokensWithReplacedSpan<I, SO>
{
}
impl<I: IntoIterator<Item: IntoTokens + WithSpan>, SO: crate::Span> WithSpan
    for IterTokensWithReplacedSpan<I, SO>
{
    type WithDefaultSpan<S: crate::Span> = Self;

    fn with_default_span<S: crate::Span>(self, _: S) -> Self::WithDefaultSpan<S> {
        self
    }

    type WithReplacedSpan<S: crate::Span> = IterTokensWithReplacedSpan<I, S>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        IterTokensWithReplacedSpan(self.0, span)
    }
}
