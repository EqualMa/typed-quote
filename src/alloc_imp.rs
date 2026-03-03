use alloc::{boxed::Box, rc::Rc};

use crate::{IntoTokens, ToTokens, sealed};

impl<T: ?Sized + IntoTokens> sealed::IntoTokens for Box<T> {}
impl<T: ?Sized + IntoTokens> IntoTokens for Box<T> {
    #[cfg(feature = "proc-macro")]
    fn into_tokens(self, tokens: &mut proc_macro::TokenStream) {
        T::box_into_tokens(self, tokens)
    }

    #[cfg(feature = "proc-macro")]
    fn into_token_stream(self) -> proc_macro::TokenStream {
        T::box_into_token_stream(self)
    }

    #[cfg(feature = "proc-macro2")]
    fn into_tokens2(self, tokens: &mut proc_macro2::TokenStream) {
        T::box_into_tokens2(self, tokens)
    }

    #[cfg(feature = "proc-macro2")]
    fn into_token_stream2(self) -> proc_macro2::TokenStream {
        T::box_into_token_stream2(self)
    }

    crate::impl_box_into_tokens! {}
}

impl<T: ?Sized + ToTokens> sealed::ToTokens for Box<T> {}
impl<T: ?Sized + ToTokens> ToTokens for Box<T> {
    #[cfg(feature = "proc-macro")]
    fn to_tokens(&self, tokens: &mut ::proc_macro::TokenStream) {
        T::to_tokens(self, tokens)
    }
    #[cfg(feature = "proc-macro")]
    fn to_token_stream(&self) -> ::proc_macro::TokenStream {
        T::to_token_stream(self)
    }

    #[cfg(feature = "proc-macro2")]
    fn to_tokens2(&self, tokens: &mut ::proc_macro2::TokenStream) {
        T::to_tokens2(self, tokens)
    }
    #[cfg(feature = "proc-macro2")]
    fn to_token_stream2(&self) -> ::proc_macro2::TokenStream {
        T::to_token_stream2(self)
    }
}

impl<T: ?Sized + ToTokens> sealed::IntoTokens for Rc<T> {}
impl<T: ?Sized + ToTokens> IntoTokens for Rc<T> {
    #[cfg(feature = "proc-macro")]
    fn into_tokens(self, tokens: &mut proc_macro::TokenStream) {
        T::to_tokens(&self, tokens)
    }

    #[cfg(feature = "proc-macro")]
    fn into_token_stream(self) -> proc_macro::TokenStream {
        T::to_token_stream(&self)
    }

    #[cfg(feature = "proc-macro2")]
    fn into_tokens2(self, tokens: &mut proc_macro2::TokenStream) {
        T::to_tokens2(&self, tokens)
    }

    #[cfg(feature = "proc-macro2")]
    fn into_token_stream2(self) -> proc_macro2::TokenStream {
        T::to_token_stream2(&self)
    }

    #[cfg(feature = "proc-macro")]
    fn box_into_tokens(self: Box<Self>, tokens: &mut ::proc_macro::TokenStream) {
        T::to_tokens(&self, tokens)
    }

    #[cfg(feature = "proc-macro")]
    fn box_into_token_stream(self: Box<Self>) -> ::proc_macro::TokenStream {
        T::to_token_stream(&self)
    }

    #[cfg(feature = "proc-macro2")]
    fn box_into_tokens2(self: Box<Self>, tokens: &mut ::proc_macro2::TokenStream) {
        T::to_tokens2(&self, tokens)
    }

    #[cfg(feature = "proc-macro2")]
    fn box_into_token_stream2(self: Box<Self>) -> ::proc_macro2::TokenStream {
        T::to_token_stream2(&self)
    }
}

impl<T: ?Sized + ToTokens> sealed::ToTokens for Rc<T> {}
impl<T: ?Sized + ToTokens> ToTokens for Rc<T> {
    #[cfg(feature = "proc-macro")]
    fn to_tokens(&self, tokens: &mut ::proc_macro::TokenStream) {
        T::to_tokens(self, tokens)
    }
    #[cfg(feature = "proc-macro")]
    fn to_token_stream(&self) -> ::proc_macro::TokenStream {
        T::to_token_stream(self)
    }

    #[cfg(feature = "proc-macro2")]
    fn to_tokens2(&self, tokens: &mut ::proc_macro2::TokenStream) {
        T::to_tokens2(self, tokens)
    }
    #[cfg(feature = "proc-macro2")]
    fn to_token_stream2(&self) -> ::proc_macro2::TokenStream {
        T::to_token_stream2(self)
    }
}
