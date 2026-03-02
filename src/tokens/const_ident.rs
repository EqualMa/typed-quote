use core::fmt;

use super::*;

impl<T: HasConstIdent + ?Sized> ConstIdent<T> {
    pub const fn new() -> Self {
        Self(PhantomData, NoSpan)
    }
}

// not public api
pub enum Underscore {}

impl HasConstIdent for Underscore {
    const IDENT: Ident<'static> = Ident("_", NoSpan);
}

impl ConstIdent<Underscore> {
    pub const UNDERSCORE: Self = Self::new();
}

impl<T: HasConstIdent + ?Sized, S: MaybeSpan> Copy for ConstIdent<T, S> {}
impl<T: HasConstIdent + ?Sized, S: MaybeSpan> Clone for ConstIdent<T, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: HasConstIdent + ?Sized, S: MaybeSpan + fmt::Debug> fmt::Debug for ConstIdent<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ConstIdent")
            .field(&format_args!("{}", T::IDENT.0))
            .field(&self.1)
            .finish()
    }
}

impl<T: HasConstIdent + ?Sized, S: MaybeSpan> sealed::IntoTokenTree for ConstIdent<T, S> {}
impl<T: HasConstIdent + ?Sized, S: MaybeSpan> IntoTokenTree for ConstIdent<T, S> {
    crate::impl_into_token_tree!(|self| Ident(T::IDENT.0, self.1).into_st());
}

impl<T: HasConstIdent + ?Sized, S: MaybeSpan> sealed::ToTokenTree for ConstIdent<T, S> {}
impl<T: HasConstIdent + ?Sized, S: MaybeSpan> ToTokenTree for ConstIdent<T, S> {
    crate::impl_to_token_tree! {copy}
}

impl<T: HasConstIdent + ?Sized, S: MaybeSpan> sealed::IntoTokens for ConstIdent<T, S> {}
impl<T: HasConstIdent + ?Sized, S: MaybeSpan> IntoTokens for ConstIdent<T, S> {
    crate::impl_into_tokens! {tt}
}

impl<T: HasConstIdent + ?Sized, S: MaybeSpan> sealed::ToTokens for ConstIdent<T, S> {}
impl<T: HasConstIdent + ?Sized, S: MaybeSpan> ToTokens for ConstIdent<T, S> {
    crate::impl_to_tokens! {copy}
}

impl<T: HasConstIdent + ?Sized, SO: MaybeSpan> sealed::WithSpan for ConstIdent<T, SO> {}
impl<T: HasConstIdent + ?Sized, SO: MaybeSpan> WithSpan for ConstIdent<T, SO> {
    type WithDefaultSpan<S: crate::Span> = ConstIdent<T, SO::WithDefaultSpan<S>>;

    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        ConstIdent(self.0, self.1.with_default_span(span))
    }

    type WithReplacedSpan<S: crate::Span> = ConstIdent<T, SO::WithReplacedSpan<S>>;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        ConstIdent(self.0, self.1.with_replaced_span(span))
    }
}

impl<T: HasConstIdent + ?Sized, SO: MaybeSpan> sealed::RefWithSpan for ConstIdent<T, SO> {}
impl<T: HasConstIdent + ?Sized, SO: MaybeSpan> RefWithSpan for ConstIdent<T, SO> {
    crate::impl_ref_with_span! {copy}
}
