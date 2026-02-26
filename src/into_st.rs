use crate::ToTokenTree;
use crate::ToTokens;
#[cfg(feature = "proc-macro2")]
use crate::maybe_span::MaybeSpan;

/// Into Stream or Tree.
pub(crate) trait IntoST<T> {
    fn into_st(self) -> T;
}

#[cfg(feature = "proc-macro")]
impl<V: ToTokens> IntoST<()> for (V, &mut proc_macro::TokenStream) {
    fn into_st(self) -> () {
        V::into_tokens(self.0, self.1)
    }
}

#[cfg(feature = "proc-macro")]
impl<V: ToTokens> IntoST<proc_macro::TokenStream> for V {
    fn into_st(self) -> proc_macro::TokenStream {
        self.into_token_stream()
    }
}

#[cfg(feature = "proc-macro")]
impl<V: ToTokenTree> IntoST<proc_macro::TokenTree> for V {
    fn into_st(self) -> proc_macro::TokenTree {
        self.into_token_tree()
    }
}

#[cfg(feature = "proc-macro")]
impl<V: MaybeSpan> IntoST<proc_macro::Span> for V {
    fn into_st(self) -> proc_macro::Span {
        self.into_span_or_call_site()
    }
}

#[cfg(feature = "proc-macro2")]
impl<V: ToTokens> IntoST<()> for (V, &mut proc_macro2::TokenStream) {
    fn into_st(self) -> () {
        V::into_tokens2(self.0, self.1)
    }
}

#[cfg(feature = "proc-macro2")]
impl<V: ToTokens> IntoST<proc_macro2::TokenStream> for V {
    fn into_st(self) -> proc_macro2::TokenStream {
        self.into_token_stream2()
    }
}

#[cfg(feature = "proc-macro2")]
impl<V: ToTokenTree> IntoST<proc_macro2::TokenTree> for V {
    fn into_st(self) -> proc_macro2::TokenTree {
        self.into_token_tree2()
    }
}

#[cfg(feature = "proc-macro2")]
impl<V: MaybeSpan> IntoST<proc_macro2::Span> for V {
    fn into_st(self) -> proc_macro2::Span {
        self.into_span2_or_call_site()
    }
}
