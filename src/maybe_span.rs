pub trait MaybeSpan: Copy + crate::sealed::MaybeSpan {
    #[cfg(feature = "proc-macro")]
    fn into_span_or_call_site(self) -> proc_macro::Span;
    #[cfg(feature = "proc-macro")]
    fn make_punct(self, punct: proc_macro::Punct) -> proc_macro::Punct;
    /// Only changes span of delimiter
    #[cfg(feature = "proc-macro")]
    fn make_group(self, g: proc_macro::Group) -> proc_macro::Group;

    #[cfg(feature = "proc-macro2")]
    fn into_span2_or_call_site(self) -> proc_macro2::Span;
    #[cfg(feature = "proc-macro2")]
    fn make_punct2(self, punct: proc_macro2::Punct) -> proc_macro2::Punct;
    /// Only changes span of delimiter
    #[cfg(feature = "proc-macro2")]
    fn make_group2(self, g: proc_macro2::Group) -> proc_macro2::Group;

    type Span: crate::Span;
    fn try_into_span(self) -> Option<Self::Span>;

    type WithDefaultSpan<S: crate::Span>: crate::Span;
    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S>;

    type WithReplacedSpan<S: crate::Span>: crate::Span;
    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S>;
}

#[cfg(feature = "proc-macro")]
impl<S: MaybeSpan> crate::into_st::IntoST<proc_macro::Punct> for (proc_macro::Punct, S) {
    fn into_st(self) -> proc_macro::Punct {
        self.1.make_punct(self.0)
    }
}

#[cfg(feature = "proc-macro2")]
impl<S: MaybeSpan> crate::into_st::IntoST<proc_macro2::Punct> for (proc_macro2::Punct, S) {
    fn into_st(self) -> proc_macro2::Punct {
        self.1.make_punct2(self.0)
    }
}

#[cfg(feature = "proc-macro")]
impl<S: MaybeSpan> crate::into_st::IntoST<proc_macro::Group> for (proc_macro::Group, S) {
    fn into_st(self) -> proc_macro::Group {
        self.1.make_group(self.0)
    }
}

#[cfg(feature = "proc-macro2")]
impl<S: MaybeSpan> crate::into_st::IntoST<proc_macro2::Group> for (proc_macro2::Group, S) {
    fn into_st(self) -> proc_macro2::Group {
        self.1.make_group2(self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NoSpan;

impl crate::sealed::MaybeSpan for NoSpan {}
impl MaybeSpan for NoSpan {
    #[cfg(feature = "proc-macro")]
    fn into_span_or_call_site(self) -> proc_macro::Span {
        proc_macro::Span::call_site()
    }
    #[cfg(feature = "proc-macro")]
    fn make_punct(self, punct: proc_macro::Punct) -> proc_macro::Punct {
        punct
    }
    #[cfg(feature = "proc-macro")]
    fn make_group(self, g: proc_macro::Group) -> proc_macro::Group {
        g
    }

    #[cfg(feature = "proc-macro2")]
    fn into_span2_or_call_site(self) -> proc_macro2::Span {
        proc_macro2::Span::call_site()
    }
    #[cfg(feature = "proc-macro2")]
    fn make_punct2(self, punct: proc_macro2::Punct) -> proc_macro2::Punct {
        punct
    }
    #[cfg(feature = "proc-macro2")]
    fn make_group2(self, g: proc_macro2::Group) -> proc_macro2::Group {
        g
    }

    type Span = crate::Never;
    fn try_into_span(self) -> Option<Self::Span> {
        None
    }

    type WithDefaultSpan<S: crate::Span> = S;

    fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
        span
    }

    type WithReplacedSpan<S: crate::Span> = S;

    fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
        span
    }
}
