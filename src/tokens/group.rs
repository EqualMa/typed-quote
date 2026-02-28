use super::*;

#[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
trait HasDelimiter<D> {
    const DELIMITER: D;
}

crate::impl_many!({
    {
        {
            use Parenthesis as G;
            #[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
            macro_rules! DELIMITER {
                ($pm:ident) => {
                    $pm::Delimiter::Parenthesis
                };
            }
        }
        {
            use Bracket as G;
            #[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
            macro_rules! DELIMITER {
                ($pm:ident) => {
                    $pm::Delimiter::Bracket
                };
            }
        }
        {
            use Brace as G;
            #[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
            macro_rules! DELIMITER {
                ($pm:ident) => {
                    $pm::Delimiter::Brace
                };
            }
        }
    }

    #[cfg(feature = "proc-macro")]
    impl HasDelimiter<proc_macro::Delimiter> for G<Empty> {
        const DELIMITER: proc_macro::Delimiter = DELIMITER!(proc_macro);
    }
    #[cfg(feature = "proc-macro2")]
    impl HasDelimiter<proc_macro2::Delimiter> for G<Empty> {
        const DELIMITER: proc_macro2::Delimiter = DELIMITER!(proc_macro2);
    }

    impl<T: IntoTokens, S: MaybeSpan> sealed::IntoTokenTree for G<T, S> {}
    impl<T: IntoTokens, S: MaybeSpan> IntoTokenTree for G<T, S> {
        crate::impl_into_token_tree!(|self| pm::TokenTree::Group(
            (
                pm::Group::new(G::<Empty>::DELIMITER, self.stream.into_st()),
                self.delimiter_span,
            )
                .into_st()
        ));
    }

    impl<T: IntoTokens, S: MaybeSpan> sealed::IntoTokens for G<T, S> {}
    impl<T: IntoTokens, S: MaybeSpan> IntoTokens for G<T, S> {
        crate::impl_into_tokens! {tt}
    }

    impl<T: ToTokens, S: MaybeSpan> sealed::ToTokenTree for G<T, S> {}
    impl<T: ToTokens, S: MaybeSpan> ToTokenTree for G<T, S> {
        crate::impl_to_token_tree!(|self| pm::TokenTree::Group(
            (
                pm::Group::new(
                    G::<Empty>::DELIMITER,
                    <&T as IntoST<pm::TokenStream>>::into_st(&self.stream)
                ),
                self.delimiter_span
            )
                .into_st()
        ));
    }

    impl<T: ToTokens, S: MaybeSpan> sealed::ToTokens for G<T, S> {}
    impl<T: ToTokens, S: MaybeSpan> ToTokens for G<T, S> {
        crate::impl_to_tokens! {tt}
    }

    impl<T: IntoTokens + WithSpan, SO: crate::Span> sealed::WithSpan for G<T, SO> {}
    impl<T: IntoTokens + WithSpan, SO: crate::Span> WithSpan for G<T, SO> {
        type WithDefaultSpan<S: crate::Span> = G<T::WithDefaultSpan<S>, SO::WithDefaultSpan<S>>;

        fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
            G {
                stream: self.stream.with_default_span(span),
                delimiter_span: self.delimiter_span.with_default_span(span),
            }
        }

        type WithReplacedSpan<S: crate::Span> = G<T::WithReplacedSpan<S>, SO::WithReplacedSpan<S>>;

        fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
            G {
                stream: self.stream.with_replaced_span(span),
                delimiter_span: self.delimiter_span.with_replaced_span(span),
            }
        }
    }

    impl<T: IntoTokens + RefWithSpan, SO: crate::Span> sealed::RefWithSpan for G<T, SO> {}
    impl<T: IntoTokens + RefWithSpan, SO: crate::Span> RefWithSpan for G<T, SO> {
        type RefWithDefaultSpan<'a, S: crate::Span>
            = G<T::RefWithDefaultSpan<'a, S>, SO::WithDefaultSpan<S>>
        where
            Self: 'a;

        fn ref_with_default_span<S: crate::Span>(
            &self,
            span: S,
        ) -> Self::RefWithDefaultSpan<'_, S> {
            G {
                stream: self.stream.ref_with_default_span(span),
                delimiter_span: self.delimiter_span.with_default_span(span),
            }
        }

        type RefWithReplacedSpan<'a, S: crate::Span>
            = G<T::RefWithReplacedSpan<'a, S>, SO::WithReplacedSpan<S>>
        where
            Self: 'a;

        fn ref_with_replaced_span<S: crate::Span>(
            &self,
            span: S,
        ) -> Self::RefWithReplacedSpan<'_, S> {
            G {
                stream: self.stream.ref_with_replaced_span(span),
                delimiter_span: self.delimiter_span.with_replaced_span(span),
            }
        }
    }
});

#[cfg(test)]
const _: () = {
    #[cfg(feature = "proc-macro")]
    {
        assert!(matches!(
            Parenthesis::DELIMITER,
            proc_macro::Delimiter::Parenthesis
        ));
        assert!(matches!(Bracket::DELIMITER, proc_macro::Delimiter::Bracket));
        assert!(matches!(Brace::DELIMITER, proc_macro::Delimiter::Brace));
    }

    #[cfg(feature = "proc-macro2")]
    {
        assert!(matches!(
            Parenthesis::DELIMITER,
            proc_macro2::Delimiter::Parenthesis
        ));
        assert!(matches!(
            Bracket::DELIMITER,
            proc_macro2::Delimiter::Bracket
        ));
        assert!(matches!(Brace::DELIMITER, proc_macro2::Delimiter::Brace));
    }
};
