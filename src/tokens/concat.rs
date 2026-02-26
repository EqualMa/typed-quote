use super::*;

impl<A: IntoTokens, B: IntoTokens> sealed::IntoTokens for Concat<A, B> {}
impl<A: IntoTokens, B: IntoTokens> IntoTokens for Concat<A, B> {
    crate::impl_into_tokens!(
        |self, ts| {
            () = (self.0, &mut *ts).into_st();
            () = (self.1, ts).into_st();
        },
        {
            let mut ts = self.0.into_st();
            () = (self.1, &mut ts).into_st();
            ts
        },
    );
}

impl<A: IntoTokens, B: IntoTokens, S: crate::Span> sealed::IntoTokens
    for ConcatWithDefaultSpan<A, B, S>
{
}
impl<A: IntoTokens + WithSpan, B: IntoTokens + WithSpan, S: crate::Span> IntoTokens
    for ConcatWithDefaultSpan<A, B, S>
{
    crate::impl_into_tokens!(
        #[proxy]
        |self| {
            let Self(a, b, span) = self;
            Concat(a.with_default_span(span), b.with_default_span(span))
        }
    );
}

impl<A: IntoTokens, B: IntoTokens, S: crate::Span> sealed::IntoTokens
    for ConcatWithReplacedSpan<A, B, S>
{
}
impl<A: IntoTokens + WithSpan, B: IntoTokens + WithSpan, S: crate::Span> IntoTokens
    for ConcatWithReplacedSpan<A, B, S>
{
    crate::impl_into_tokens!(
        #[proxy]
        |self| {
            let Self(a, b, span) = self;
            Concat(a.with_replaced_span(span), b.with_replaced_span(span))
        }
    );
}

impl<A: ToTokens, B: ToTokens> sealed::ToTokens for Concat<A, B> {}
impl<A: ToTokens, B: ToTokens> ToTokens for Concat<A, B> {
    crate::impl_to_tokens!(
        |self, ts| {
            () = (&self.0, &mut *ts).into_st();
            () = (&self.1, ts).into_st();
        },
        {
            let mut ts = (&self.0).into_st();
            () = (&self.1, &mut ts).into_st();
            ts
        },
    );
}

impl<A: ToTokens, B: ToTokens, S: crate::Span> sealed::ToTokens for ConcatWithDefaultSpan<A, B, S> {}
impl<A: ToTokens + RefWithSpan, B: ToTokens + RefWithSpan, S: crate::Span> ToTokens
    for ConcatWithDefaultSpan<A, B, S>
{
    crate::impl_to_tokens!(
        #[proxy]
        |self| {
            let Self(a, b, span) = self;
            let span = *span;
            Concat(a.ref_with_default_span(span), b.ref_with_default_span(span))
        }
    );
}

impl<A: ToTokens, B: ToTokens, S: crate::Span> sealed::ToTokens
    for ConcatWithReplacedSpan<A, B, S>
{
}
impl<A: ToTokens + RefWithSpan, B: ToTokens + RefWithSpan, S: crate::Span> ToTokens
    for ConcatWithReplacedSpan<A, B, S>
{
    crate::impl_to_tokens!(
        #[proxy]
        |self| {
            let Self(a, b, span) = self;
            let span = *span;
            Concat(
                a.ref_with_replaced_span(span),
                b.ref_with_replaced_span(span),
            )
        }
    );
}

impl<A: WithSpan, B: WithSpan> sealed::WithSpan for Concat<A, B> {}
impl<A: WithSpan, B: WithSpan> WithSpan for Concat<A, B> {
    type WithDefaultSpan<S: crate::Span> = ConcatWithDefaultSpan<A, B, S>;

    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        ConcatWithDefaultSpan(self.0, self.1, span)
    }

    type WithReplacedSpan<S: crate::Span> = ConcatWithReplacedSpan<A, B, S>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        ConcatWithReplacedSpan(self.0, self.1, span)
    }
}

impl<A: WithSpan, B: WithSpan, SO: crate::Span> sealed::WithSpan
    for ConcatWithDefaultSpan<A, B, SO>
{
}
impl<A: WithSpan, B: WithSpan, SO: crate::Span> WithSpan for ConcatWithDefaultSpan<A, B, SO> {
    type WithDefaultSpan<S: crate::Span> = Self;

    fn with_default_span<S: crate::Span>(self, _: S) -> Self::WithDefaultSpan<S> {
        self
    }

    type WithReplacedSpan<S: crate::Span> = ConcatWithReplacedSpan<A, B, S>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        let Self(a, b, _) = self;
        ConcatWithReplacedSpan(a, b, span)
    }
}

impl<A: WithSpan, B: WithSpan, SO: crate::Span> sealed::WithSpan
    for ConcatWithReplacedSpan<A, B, SO>
{
}
impl<A: WithSpan, B: WithSpan, SO: crate::Span> WithSpan for ConcatWithReplacedSpan<A, B, SO> {
    type WithDefaultSpan<S: crate::Span> = Self;

    fn with_default_span<S: crate::Span>(self, _: S) -> Self::WithDefaultSpan<S> {
        self
    }

    type WithReplacedSpan<S: crate::Span> = ConcatWithReplacedSpan<A, B, S>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        let Self(a, b, _) = self;
        ConcatWithReplacedSpan(a, b, span)
    }
}

impl<A: RefWithSpan, B: RefWithSpan> sealed::RefWithSpan for Concat<A, B> {}
impl<A: RefWithSpan, B: RefWithSpan> RefWithSpan for Concat<A, B> {
    type RefWithDefaultSpan<'a, S: crate::Span>
        = ConcatWithDefaultSpan<&'a A, &'a B, S>
    where
        Self: 'a;

    fn ref_with_default_span<S: crate::Span>(&self, span: S) -> Self::RefWithDefaultSpan<'_, S> {
        let Self(a, b) = self;
        ConcatWithDefaultSpan(a, b, span)
    }

    type RefWithReplacedSpan<'a, S: crate::Span>
        = ConcatWithReplacedSpan<&'a A, &'a B, S>
    where
        Self: 'a;

    fn ref_with_replaced_span<S: crate::Span>(&self, span: S) -> Self::RefWithReplacedSpan<'_, S> {
        let Self(a, b) = self;
        ConcatWithReplacedSpan(a, b, span)
    }
}

impl<A: RefWithSpan, B: RefWithSpan, S: crate::Span> sealed::RefWithSpan
    for ConcatWithDefaultSpan<A, B, S>
{
}
impl<A: RefWithSpan, B: RefWithSpan, SO: crate::Span> RefWithSpan
    for ConcatWithDefaultSpan<A, B, SO>
{
    type RefWithDefaultSpan<'a, S: crate::Span>
        = &'a Self
    where
        Self: 'a;

    fn ref_with_default_span<S: crate::Span>(&self, _: S) -> Self::RefWithDefaultSpan<'_, S> {
        self
    }

    type RefWithReplacedSpan<'a, S: crate::Span>
        = ConcatWithReplacedSpan<&'a A, &'a B, S>
    where
        Self: 'a;

    fn ref_with_replaced_span<S: crate::Span>(&self, span: S) -> Self::RefWithReplacedSpan<'_, S> {
        let Self(a, b, _) = self;
        ConcatWithReplacedSpan(a, b, span)
    }
}

impl<A: RefWithSpan, B: RefWithSpan, S: crate::Span> sealed::RefWithSpan
    for ConcatWithReplacedSpan<A, B, S>
{
}
impl<A: RefWithSpan, B: RefWithSpan, SO: crate::Span> RefWithSpan
    for ConcatWithReplacedSpan<A, B, SO>
{
    type RefWithDefaultSpan<'a, S: crate::Span>
        = &'a Self
    where
        Self: 'a;

    fn ref_with_default_span<S: crate::Span>(&self, _: S) -> Self::RefWithDefaultSpan<'_, S> {
        self
    }

    type RefWithReplacedSpan<'a, S: crate::Span>
        = ConcatWithReplacedSpan<&'a A, &'a B, S>
    where
        Self: 'a;

    fn ref_with_replaced_span<S: crate::Span>(&self, span: S) -> Self::RefWithReplacedSpan<'_, S> {
        let Self(a, b, _) = self;
        ConcatWithReplacedSpan(a, b, span)
    }
}
