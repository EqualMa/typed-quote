use crate::{
    IntoTokenTree, IntoTokens, RefWithSpan, Span, ToTokenTree, ToTokens, WithSpan, into_st::IntoST,
    maybe_span::MaybeSpan, replace_span_of::ReplaceSpanOf, sealed,
};

trait MyInto<S> {
    fn my_into(self) -> S;
}

impl<T> MyInto<T> for T {
    fn my_into(self) -> T {
        self
    }
}

#[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
trait IdentIsDollarCrate {
    fn ident_is_dollar_crate(&self) -> bool;
}

#[cfg(feature = "proc-macro")]
#[cfg(feature = "proc-macro2")]
mod convert_12;

#[cfg(feature = "proc-macro")]
mod proc_macro_1;
#[cfg(feature = "proc-macro2")]
mod proc_macro_2;

mod replace_span_of_ref;

crate::impl_many!({
    {
        #[cfg(feature = "proc-macro")]
        {
            use proc_macro as pmn;
        }
        #[cfg(feature = "proc-macro2")]
        {
            use proc_macro2 as pmn;
        }
    }

    impl sealed::IntoTokens for pmn::TokenStream {}
    #[allow(clippy::useless_conversion)]
    impl IntoTokens for pmn::TokenStream {
        crate::impl_into_tokens!(
            |self, ts| {
                ts.extend(pm::TokenStream::from(self));
            },
            self.into(),
        );
    }

    impl sealed::ToTokens for pmn::TokenStream {}
    impl ToTokens for pmn::TokenStream {
        crate::impl_to_tokens!(
            #[proxy]
            |self| self.clone()
        );
    }

    impl sealed::WithSpan for pmn::TokenStream {}
    impl WithSpan for pmn::TokenStream {
        type WithDefaultSpan<S: Span> = Self;

        fn with_default_span<S: Span>(self, _: S) -> Self::WithDefaultSpan<S> {
            self
        }

        type WithReplacedSpan<S: Span> = <S as ReplaceSpanOf<Self>>::ReplaceSpanOf;

        fn with_replaced_span<S: Span>(self, span: S) -> Self::WithReplacedSpan<S> {
            span.replace_span_of(self)
        }
    }

    impl sealed::RefWithSpan for pmn::TokenStream {}
    impl RefWithSpan for pmn::TokenStream {
        type RefWithDefaultSpan<'a, S: Span>
            = &'a Self
        where
            Self: 'a;

        fn ref_with_default_span<S: Span>(&self, _: S) -> Self::RefWithDefaultSpan<'_, S> {
            self
        }

        type RefWithReplacedSpan<'a, S: Span>
            = replace_span_of_ref::ReplaceSpanOfRef<'a, Self, S>
        where
            Self: 'a;

        fn ref_with_replaced_span<S: Span>(&self, span: S) -> Self::RefWithReplacedSpan<'_, S> {
            replace_span_of_ref::ReplaceSpanOfRef(self, span)
        }
    }

    // token tree
    crate::impl_many!({
        {
            {
                use pmn::TokenTree as TT;
            }
            {
                use pmn::Group as TT;
            }
            {
                use pmn::Ident as TT;
            }
            {
                use pmn::Punct as TT;
            }
            {
                use pmn::Literal as TT;
            }
        }

        impl sealed::IntoTokenTree for TT {}
        #[allow(clippy::useless_conversion)]
        impl IntoTokenTree for TT {
            crate::impl_into_token_tree!(|self| pmn::TokenTree::from(self).my_into());
        }

        impl sealed::ToTokenTree for TT {}
        impl ToTokenTree for TT {
            crate::impl_to_token_tree!(|self| self.clone().into_st());
        }

        impl sealed::IntoTokens for TT {}
        impl IntoTokens for TT {
            crate::impl_into_tokens! {tt}
        }

        impl sealed::ToTokens for TT {}
        impl ToTokens for TT {
            crate::impl_to_tokens! {tt}
        }

        impl sealed::WithSpan for TT {}
        impl WithSpan for TT {
            type WithDefaultSpan<S: Span> = Self;

            fn with_default_span<S: Span>(self, _: S) -> Self::WithDefaultSpan<S> {
                self
            }

            type WithReplacedSpan<S: Span> = <S as ReplaceSpanOf<TT>>::ReplaceSpanOf;

            fn with_replaced_span<S: Span>(self, span: S) -> Self::WithReplacedSpan<S> {
                span.replace_span_of(self)
            }
        }

        impl sealed::RefWithSpan for TT {}
        impl RefWithSpan for TT {
            type RefWithDefaultSpan<'a, S: Span>
                = &'a Self
            where
                Self: 'a;

            fn ref_with_default_span<S: Span>(&self, _: S) -> Self::RefWithDefaultSpan<'_, S> {
                self
            }

            type RefWithReplacedSpan<'a, S: Span>
                = replace_span_of_ref::ReplaceSpanOfRef<'a, Self, S>
            where
                Self: 'a;

            fn ref_with_replaced_span<S: Span>(&self, span: S) -> Self::RefWithReplacedSpan<'_, S> {
                replace_span_of_ref::ReplaceSpanOfRef(self, span)
            }
        }
    });

    // span
    impl sealed::MaybeSpan for pmn::Span {}
    impl MaybeSpan for pmn::Span {
        #[cfg(feature = "proc-macro")]
        fn into_span_or_call_site(self) -> proc_macro::Span {
            self.my_into()
        }
        #[cfg(feature = "proc-macro")]
        fn make_punct(self, mut punct: proc_macro::Punct) -> proc_macro::Punct {
            punct.set_span(self.my_into());
            punct
        }
        #[cfg(feature = "proc-macro")]
        fn make_group(self, mut g: proc_macro::Group) -> proc_macro::Group {
            g.set_span(self.my_into());
            g
        }
        #[cfg(feature = "proc-macro")]
        fn make_literal(self, mut literal: proc_macro::Literal) -> proc_macro::Literal {
            literal.set_span(self.my_into());
            literal
        }

        #[cfg(feature = "proc-macro2")]
        fn into_span2_or_call_site(self) -> proc_macro2::Span {
            #[allow(clippy::useless_conversion)]
            self.into()
        }
        #[cfg(feature = "proc-macro2")]
        fn make_punct2(self, mut punct: proc_macro2::Punct) -> proc_macro2::Punct {
            #[allow(clippy::useless_conversion)]
            punct.set_span(self.into());
            punct
        }
        #[cfg(feature = "proc-macro2")]
        fn make_group2(self, mut g: proc_macro2::Group) -> proc_macro2::Group {
            #[allow(clippy::useless_conversion)]
            g.set_span(self.into());
            g
        }
        #[cfg(feature = "proc-macro2")]
        fn make_literal2(self, mut literal: proc_macro2::Literal) -> proc_macro2::Literal {
            #[allow(clippy::useless_conversion)]
            literal.set_span(self.into());
            literal
        }

        type Span = Self;
        fn try_into_span(self) -> Option<Self::Span> {
            Some(self)
        }

        type WithDefaultSpan<S: crate::Span> = Self;
        fn with_default_span<S: crate::Span>(self, _: S) -> Self::WithDefaultSpan<S> {
            self
        }

        type WithReplacedSpan<S: crate::Span> = S;
        fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
            span
        }
    }

    impl sealed::Span for pmn::Span {}
    impl Span for pmn::Span {}
});
