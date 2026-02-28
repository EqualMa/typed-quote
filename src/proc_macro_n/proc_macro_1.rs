use super::*;

impl IdentIsDollarCrate for proc_macro::Ident {
    fn ident_is_dollar_crate(&self) -> bool {
        self.to_string() == "$crate"
    }
}

crate::impl_many!({
    {
        {
            use proc_macro as pmn;
        }
        #[cfg(feature = "proc-macro2")]
        {
            use proc_macro2 as pmn;
        }
    }

    impl ReplaceSpanOf<pmn::TokenStream> for proc_macro::Span {
        type ReplaceSpanOf = pmn::TokenStream;

        fn replace_span_of(self, t: pmn::TokenStream) -> Self::ReplaceSpanOf {
            t.into_iter().map(|tt| self.replace_span_of(tt)).collect()
        }
    }

    impl ReplaceSpanOf<pmn::TokenTree> for proc_macro::Span {
        type ReplaceSpanOf = pmn::TokenTree;
        fn replace_span_of(self, tt: pmn::TokenTree) -> Self::ReplaceSpanOf {
            match tt {
                pmn::TokenTree::Group(group) => pmn::TokenTree::Group(self.replace_span_of(group)),
                pmn::TokenTree::Ident(ident) => pmn::TokenTree::Ident(self.replace_span_of(ident)),
                mut tt => {
                    tt.set_span(self.into());
                    tt
                }
            }
        }
    }

    impl ReplaceSpanOf<pmn::Group> for proc_macro::Span {
        type ReplaceSpanOf = pmn::Group;

        fn replace_span_of(self, group: pmn::Group) -> Self::ReplaceSpanOf {
            let span: pmn::Span = self.into();
            let mut group =
                pmn::Group::new(group.delimiter(), self.replace_span_of(group.stream()));
            group.set_span(span);

            group
        }
    }

    impl ReplaceSpanOf<pmn::Ident> for proc_macro::Span {
        type ReplaceSpanOf = pmn::Ident;
        fn replace_span_of(self, mut tt: pmn::Ident) -> Self::ReplaceSpanOf {
            if tt.ident_is_dollar_crate() {
                return tt;
            }
            tt.set_span(self.into());
            tt
        }
    }

    crate::impl_many!({
        {
            {
                use pmn::Punct as TT;
            }
            {
                use pmn::Literal as TT;
            }
        }
        impl ReplaceSpanOf<TT> for proc_macro::Span {
            type ReplaceSpanOf = TT;
            fn replace_span_of(self, mut tt: TT) -> Self::ReplaceSpanOf {
                tt.set_span(self.into());
                tt
            }
        }
    });
});
