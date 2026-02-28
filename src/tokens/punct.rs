use super::*;

macro_rules! imps {
    (
        [$($Punct:ident)+]
        $This:ident
        $imp:tt
    ) => {
        $(
            const _: () = {
                use self::$Punct as $This;

                crate::expand_or! {$imp}
            };
        )+
    };
}

macro_rules! punct {
    ({
        #$attr:tt
        [
            $($Punct:ident ($PUNCT_value:literal)),+ $(,)?
        ];

        use $This:ident;

        $($imp:tt)*
    }) => {
        $(
            #$attr
            pub struct $Punct<S: MaybeSpan>(pub S);

            #[cfg(any(feature = "proc-macro", feature = "proc-macro2"))]
            impl $Punct<NoSpan> {
                const CHAR: char = $PUNCT_value;
            }
        )+
        imps! {
            [$($Punct)+]
            $This
            [$($imp)*]
        }
    };
}

punct!({
    #[derive(Debug, Clone, Copy)]
    [
        Pound('#'),
        Comma(','),
        Dot('.'),
        Semi(';'),
        Colon(':'),
        Add('+'),
        And('&'),
        At('@'),
        Bang('!'),
        Caret('^'),
        Div('/'),
        Eq('='),
        Gt('>'),
        Lt('<'),
        Or('|'),
        Question('?'),
        Rem('%'),
        Star('*'),
        Sub('-'),
    ];

    use PUNCT;

    impl<S: MaybeSpan> sealed::IntoTokenTree for PUNCT<S> {}
    impl<S: MaybeSpan> IntoTokenTree for PUNCT<S> {
        crate::impl_into_token_tree!(|self| pm::TokenTree::Punct(
            (
                pm::Punct::new(PUNCT::<NoSpan>::CHAR, pm::Spacing::Alone),
                self.0
            )
                .into_st()
        ));
    }

    impl<S: MaybeSpan> sealed::ToTokenTree for PUNCT<S> {}
    impl<S: MaybeSpan> ToTokenTree for PUNCT<S> {
        crate::impl_to_token_tree! {copy}
    }

    impl<S: MaybeSpan> sealed::IntoTokens for PUNCT<S> {}
    impl<S: MaybeSpan> IntoTokens for PUNCT<S> {
        crate::impl_into_tokens! {tt}
    }

    impl<S: MaybeSpan> sealed::ToTokens for PUNCT<S> {}
    impl<S: MaybeSpan> ToTokens for PUNCT<S> {
        crate::impl_to_tokens! {copy}
    }

    impl<SO: MaybeSpan> sealed::WithSpan for PUNCT<SO> {}
    impl<SO: MaybeSpan> WithSpan for PUNCT<SO> {
        type WithDefaultSpan<S: crate::Span> = PUNCT<SO::WithDefaultSpan<S>>;

        fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
            PUNCT(self.0.with_default_span(span))
        }

        type WithReplacedSpan<S: crate::Span> = PUNCT<SO::WithReplacedSpan<S>>;

        fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
            PUNCT(self.0.with_replaced_span(span))
        }
    }

    impl<SO: MaybeSpan> sealed::RefWithSpan for PUNCT<SO> {}
    impl<SO: MaybeSpan> RefWithSpan for PUNCT<SO> {
        crate::impl_ref_with_span! {copy}
    }
});
