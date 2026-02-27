use super::*;

trait Puncts<TS> {
    type Puncts;
    fn puncts(span: impl MaybeSpan) -> Self::Puncts;
}

macro_rules! puncts {
    (($p0:literal, $p1:literal, $p2:literal $(,)?) $span:ident) => {
        puncts![@[$p0 $p1][$p2] $span]
    };
    (($p0:literal, $p1:literal $(,)?) $span:ident) => {
        puncts![@[$p0][$p1] $span]
    };
    (@[$($ch:literal)+][$last:literal] $span:ident) => {
        puncts![
            @@[
                $(($ch Joint))+
                ($last Alone)
            ]
            $span
        ]
    };
    (@@[$(($ch:literal $spacing:ident))+] $span:ident) => {
        [$(
            pm::TokenTree::Punct(
                (
                    pm::Punct::new($ch, pm::Spacing::$spacing),
                    $span,
                ).into_st()
            ),
        )+]
    };
}

macro_rules! count_literals {
    ($($a:literal),+ $(,)?) => {
        const { [$($a,)+].len() }
    };
}

macro_rules! define_struct {
    ({$(#$attrs:tt)*} $PunctGroup:ident) => {
        $(#$attrs)*
        pub struct $PunctGroup<S: MaybeSpan>(pub S);
    };
}

macro_rules! define_structs {
    ($attrs:tt [$($PunctGroup:ident)*]) => {
        $(
            define_struct! {
                $attrs
                $PunctGroup
            }
        )*
    };
}

macro_rules! imp {
    ({
        $(#$attrs:tt)*
        [$(
            $PunctGroup:ident $chars:tt
        ),+ $(,)?];

        use $PS:ident;

        $($imp:tt)*
    }) => {
        define_structs! {
            {
                $(#$attrs)*
            }
            [$($PunctGroup)+]
        }

        $(
            crate::impl_many!({
                {
                    #[cfg(feature = "proc-macro")]
                    {
                        use proc_macro as pm;
                    }
                    #[cfg(feature = "proc-macro2")]
                    {
                        use proc_macro2 as pm;
                    }
                }

                impl<S: MaybeSpan> Puncts<pm::TokenStream> for $PunctGroup<S> {
                    type Puncts = [pm::TokenTree; count_literals! $chars];
                    fn puncts(span: impl MaybeSpan) -> Self::Puncts {
                        puncts!($chars span)
                    }
                }
            });
        )+

        imp! {
            @[$($PunctGroup)+]
            $PS
            [$($imp)*]
        }
    };
    (
        @[$($PunctGroup:ident)+]
        $PS:ident
        $imp:tt
    ) => {
        $(
            const _: () = {
                use $PunctGroup as $PS;

                crate::expand_or! {$imp}
            };
        )+
    };
}

imp!({
    #[derive(Debug, Clone, Copy)]
    [
        Colon2(':', ':'),
        AddEq('+', '='),
        AndAnd('&', '&'),
        AndEq('&', '='),
        CaretEq('^', '='),
        DivEq('/', '='),
        Dot2('.', '.'),
        Dot3('.', '.', '.'),
        DotDotEq('.', '.', '='),
        EqEq('=', '='),
        Ge('>', '='),
        Le('<', '='),
        MulEq('*', '='),
        Ne('!', '='),
        OrEq('|', '='),
        OrOr('|', '|'),
        RArrow('-', '>'),
        LArrow('<', '-'),
        RemEq('%', '='),
        FatArrow('=', '>'),
        Shl('<', '<'),
        ShlEq('<', '<', '='),
        Shr('>', '>'),
        ShrEq('>', '>', '='),
        SubEq('-', '='),
    ];
    use PS;

    impl<S: MaybeSpan> sealed::IntoTokens for PS<S> {}
    impl<S: MaybeSpan> IntoTokens for PS<S> {
        crate::impl_into_tokens!(
            |self, ts| ts.extend(<Self as Puncts<pm::TokenStream>>::puncts(self.0)),
            pm::TokenStream::from_iter(<Self as Puncts<pm::TokenStream>>::puncts(self.0)),
        );
    }

    impl<S: MaybeSpan> sealed::ToTokens for PS<S> {}
    impl<S: MaybeSpan> ToTokens for PS<S> {
        crate::impl_to_tokens! {copy}
    }

    impl<SO: MaybeSpan> sealed::WithSpan for PS<SO> {}
    impl<SO: MaybeSpan> WithSpan for PS<SO> {
        type WithDefaultSpan<S: crate::Span> = PS<SO::WithDefaultSpan<S>>;

        fn with_default_span<S: crate::Span>(self, span: S) -> Self::WithDefaultSpan<S> {
            PS(self.0.with_default_span(span))
        }

        type WithReplacedSpan<S: crate::Span> = PS<SO::WithReplacedSpan<S>>;

        fn with_replaced_span<S: crate::Span>(self, span: S) -> Self::WithReplacedSpan<S> {
            PS(self.0.with_replaced_span(span))
        }
    }

    impl<SO: MaybeSpan> sealed::RefWithSpan for PS<SO> {}
    impl<SO: MaybeSpan> RefWithSpan for PS<SO> {
        crate::impl_ref_with_span! {copy}
    }
});
