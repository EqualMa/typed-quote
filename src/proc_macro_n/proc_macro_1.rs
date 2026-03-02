use core::fmt::{self};

use super::*;

/// only store first N bytes
struct Buf<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> Buf<N> {
    fn write_bytes(&mut self, s: &[u8]) {
        if s.is_empty() {
            return;
        }

        'push: {
            let rest = match self.buf.split_at_mut_checked(self.len) {
                Some((_, rest)) if rest.len() > 0 => rest,
                _ => {
                    break 'push;
                }
            };

            let (push_to, push_from) = if rest.len() > s.len() {
                let (push_to, _) = rest.split_at_mut(s.len());
                (push_to, s)
            } else {
                let (push_from, _) = s.split_at(rest.len());
                (rest, push_from)
            };

            push_to.copy_from_slice(push_from);
        }

        self.len += s.len();
    }
}

impl<const N: usize> fmt::Write for Buf<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Ok(self.write_bytes(s.as_bytes()))
    }
}

fn is_dollar_crate(v: impl fmt::Display) -> bool {
    const DOLLAR_CRATE: [u8; 6] = *b"$crate";
    let mut buf = Buf {
        buf: [0u8; DOLLAR_CRATE.len()],
        len: 0,
    };

    {
        use core::fmt::Write as _;
        write!(buf, "{}", v).unwrap();
    }

    buf.len == const { DOLLAR_CRATE.len() } && matches!(buf.buf, DOLLAR_CRATE)
}

impl IdentIsDollarCrate for proc_macro::Ident {
    fn ident_is_dollar_crate(&self) -> bool {
        is_dollar_crate(self)
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

#[cfg(test)]
mod tests;
