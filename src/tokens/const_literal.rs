use core::fmt;

use super::*;

impl<T: HasConstLiteral + ?Sized> Default for ConstLiteral<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: HasConstLiteral + ?Sized> ConstLiteral<T> {
    pub const fn new() -> Self {
        Self(PhantomData, NoSpan)
    }
}

impl<T: HasConstLiteral + ?Sized, S: MaybeSpan> ConstLiteral<T, S> {
    pub const fn as_literal(self) -> Literal<'static, S> {
        Literal(T::LITERAL.0, self.1)
    }
}

impl<T: HasConstLiteral + ?Sized, S: MaybeSpan> Copy for ConstLiteral<T, S> {}
impl<T: HasConstLiteral + ?Sized, S: MaybeSpan> Clone for ConstLiteral<T, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: HasConstLiteral + ?Sized, S: MaybeSpan + fmt::Debug> fmt::Debug for ConstLiteral<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ConstLiteral")
            .field(&format_args!("{}", T::LITERAL.0))
            .field(&self.1)
            .finish()
    }
}

impl<T: HasConstLiteral + ?Sized, S: MaybeSpan> sealed::IntoTokenTree for ConstLiteral<T, S> {}
impl<T: HasConstLiteral + ?Sized, S: MaybeSpan> IntoTokenTree for ConstLiteral<T, S> {
    crate::impl_into_token_tree!(|self| Literal(T::LITERAL.0, self.1).into_st());
}

impl<T: HasConstLiteral + ?Sized, S: MaybeSpan> sealed::ToTokenTree for ConstLiteral<T, S> {}
impl<T: HasConstLiteral + ?Sized, S: MaybeSpan> ToTokenTree for ConstLiteral<T, S> {
    crate::impl_to_token_tree! {copy}
}

impl<T: HasConstLiteral + ?Sized, S: MaybeSpan> sealed::IntoTokens for ConstLiteral<T, S> {}
impl<T: HasConstLiteral + ?Sized, S: MaybeSpan> IntoTokens for ConstLiteral<T, S> {
    crate::impl_into_tokens! {tt}
}

impl<T: HasConstLiteral + ?Sized, S: MaybeSpan> sealed::ToTokens for ConstLiteral<T, S> {}
impl<T: HasConstLiteral + ?Sized, S: MaybeSpan> ToTokens for ConstLiteral<T, S> {
    crate::impl_to_tokens! {copy}
}

impl<T: HasConstLiteral + ?Sized, SO: MaybeSpan> sealed::WithSpan for ConstLiteral<T, SO> {}
impl<T: HasConstLiteral + ?Sized, SO: MaybeSpan> WithSpan for ConstLiteral<T, SO> {
    type WithDefaultSpan<S: crate::Span> = ConstLiteral<T, SO::WithDefaultSpan<S>>;

    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        ConstLiteral(self.0, self.1.with_default_span(span))
    }

    type WithReplacedSpan<S: crate::Span> = ConstLiteral<T, SO::WithReplacedSpan<S>>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        ConstLiteral(self.0, self.1.with_replaced_span(span))
    }
}

impl<T: HasConstLiteral + ?Sized, SO: MaybeSpan> sealed::RefWithSpan for ConstLiteral<T, SO> {}
impl<T: HasConstLiteral + ?Sized, SO: MaybeSpan> RefWithSpan for ConstLiteral<T, SO> {
    crate::impl_ref_with_span! {copy}
}
