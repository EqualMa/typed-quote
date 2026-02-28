use proc_macro as pm1;
use proc_macro2 as pm2;

use super::MyInto;

impl MyInto<pm1::Span> for pm2::Span {
    fn my_into(self) -> pm1::Span {
        self.unwrap()
    }
}

crate::impl_many!({
    {
        {
            use pm1 as pmx;
            use pm2 as pmy;
        }
        {
            use pm1 as pmy;
            use pm2 as pmx;
        }
    }

    impl MyInto<pmy::TokenStream> for pmx::TokenStream {
        fn my_into(self) -> pmy::TokenStream {
            self.into()
        }
    }

    impl MyInto<pmy::TokenTree> for pmx::TokenTree {
        fn my_into(self) -> pmy::TokenTree {
            let ts = pmy::TokenStream::from(pmx::TokenStream::from(self));

            let mut ts = ts.into_iter();

            let tt = ts.next().unwrap();

            assert!(ts.next().is_none());

            tt
        }
    }
});

impl MyInto<proc_macro2::Group> for proc_macro::Group {
    fn my_into(self) -> proc_macro2::Group {
        let mut g = proc_macro2::Group::new(
            match self.delimiter() {
                proc_macro::Delimiter::Parenthesis => proc_macro2::Delimiter::Parenthesis,
                proc_macro::Delimiter::Brace => proc_macro2::Delimiter::Brace,
                proc_macro::Delimiter::Bracket => proc_macro2::Delimiter::Bracket,
                proc_macro::Delimiter::None => proc_macro2::Delimiter::None,
            },
            self.stream().into(),
        );

        g.set_span(self.span().into());

        g
    }
}

crate::impl_many!({
    {
        {
            use proc_macro::Ident as Pm1TT;
            use proc_macro::TokenTree::Ident as Pm1TTVar;
            use proc_macro2::Ident as Pm2TT;
            use proc_macro2::TokenTree::Ident as Pm2TTVar;
        }
        {
            use proc_macro::Punct as Pm1TT;
            use proc_macro::TokenTree::Punct as Pm1TTVar;
            use proc_macro2::Punct as Pm2TT;
            use proc_macro2::TokenTree::Punct as Pm2TTVar;
        }
        {
            use proc_macro::Literal as Pm1TT;
            use proc_macro::TokenTree::Literal as Pm1TTVar;
            use proc_macro2::Literal as Pm2TT;
            use proc_macro2::TokenTree::Literal as Pm2TTVar;
        }
    }

    impl MyInto<Pm2TT> for Pm1TT {
        fn my_into(self) -> Pm2TT {
            let tt: proc_macro2::TokenTree = Pm1TTVar(self).my_into();
            match tt {
                Pm2TTVar(ident) => ident,
                _ => unreachable!(),
            }
        }
    }
});
