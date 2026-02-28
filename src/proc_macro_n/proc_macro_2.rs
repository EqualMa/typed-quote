use super::*;

crate::impl_many!({
    {
        #[cfg(feature = "proc-macro")]
        {
            use proc_macro as pmn;
        }
        {
            use proc_macro2 as pmn;
        }
    }

    impl ReplaceSpanOf<pmn::TokenStream> for proc_macro2::Span {
        type ReplaceSpanOf = proc_macro2::TokenStream;

        fn replace_span_of(self, t: pmn::TokenStream) -> Self::ReplaceSpanOf {
            proc_macro2::TokenStream::from(t)
                .into_iter()
                .map(|tt| self.replace_span_of(tt))
                .collect()
        }
    }

    impl ReplaceSpanOf<pmn::TokenTree> for proc_macro2::Span {
        type ReplaceSpanOf = proc_macro2::TokenTree;
        fn replace_span_of(self, tt: pmn::TokenTree) -> Self::ReplaceSpanOf {
            let tt: proc_macro2::TokenTree = tt.my_into();
            match tt {
                proc_macro2::TokenTree::Group(group) => {
                    proc_macro2::TokenTree::Group(self.replace_span_of(group))
                }
                mut tt => {
                    tt.set_span(self.into());
                    tt
                }
            }
        }
    }

    impl ReplaceSpanOf<pmn::Group> for proc_macro2::Span {
        type ReplaceSpanOf = proc_macro2::Group;

        fn replace_span_of(self, group: pmn::Group) -> Self::ReplaceSpanOf {
            let group: proc_macro2::Group = group.my_into();
            let mut group =
                proc_macro2::Group::new(group.delimiter(), self.replace_span_of(group.stream()));
            group.set_span(self);

            group
        }
    }

    crate::impl_many!({
        {
            {
                use pmn::Ident as TT;
                use proc_macro2::Ident as TTReplaced;
            }
            {
                use pmn::Punct as TT;
                use proc_macro2::Punct as TTReplaced;
            }
            {
                use pmn::Literal as TT;
                use proc_macro2::Literal as TTReplaced;
            }
        }
        impl ReplaceSpanOf<TT> for proc_macro2::Span {
            type ReplaceSpanOf = TTReplaced;
            fn replace_span_of(self, tt: TT) -> Self::ReplaceSpanOf {
                let mut tt: TTReplaced = tt.my_into();
                tt.set_span(self);
                tt
            }
        }
    });
});
