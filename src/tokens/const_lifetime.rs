use core::fmt;

use super::*;

impl<T: HasConstLifetime + ?Sized> ConstLifetime<T> {
    pub const fn new() -> Self {
        Self(PhantomData, NoSpan)
    }
}

impl<T: HasConstLifetime + ?Sized, S: MaybeSpan> Copy for ConstLifetime<T, S> {}
impl<T: HasConstLifetime + ?Sized, S: MaybeSpan> Clone for ConstLifetime<T, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: HasConstLifetime + ?Sized, S: MaybeSpan + fmt::Debug> fmt::Debug for ConstLifetime<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ConstLifetime")
            .field(&format_args!("{}", T::LIFETIME.0))
            .field(&self.1)
            .finish()
    }
}

impl<T: HasConstLifetime + ?Sized, S: MaybeSpan> sealed::IntoTokens for ConstLifetime<T, S> {}
impl<T: HasConstLifetime + ?Sized, S: MaybeSpan> IntoTokens for ConstLifetime<T, S> {
    crate::impl_into_tokens!(|self, ts| (Lifetime(T::LIFETIME.0, self.1), ts).into_st());
}

impl<T: HasConstLifetime + ?Sized, S: MaybeSpan> sealed::ToTokens for ConstLifetime<T, S> {}
impl<T: HasConstLifetime + ?Sized, S: MaybeSpan> ToTokens for ConstLifetime<T, S> {
    crate::impl_to_tokens! {copy}
}

impl<T: HasConstLifetime + ?Sized, SO: MaybeSpan> sealed::WithSpan for ConstLifetime<T, SO> {}
impl<T: HasConstLifetime + ?Sized, SO: MaybeSpan> WithSpan for ConstLifetime<T, SO> {
    type WithDefaultSpan<S: crate::Span> = ConstLifetime<T, SO::WithDefaultSpan<S>>;

    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        ConstLifetime(self.0, self.1.with_default_span(span))
    }

    type WithReplacedSpan<S: crate::Span> = ConstLifetime<T, SO::WithReplacedSpan<S>>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        ConstLifetime(self.0, self.1.with_replaced_span(span))
    }
}

impl<T: HasConstLifetime + ?Sized, SO: MaybeSpan> sealed::RefWithSpan for ConstLifetime<T, SO> {}
impl<T: HasConstLifetime + ?Sized, SO: MaybeSpan> RefWithSpan for ConstLifetime<T, SO> {
    crate::impl_ref_with_span! {copy}
}
