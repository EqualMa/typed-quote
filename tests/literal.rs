#![cfg(feature = "proc-macro2")]

use typed_quote::{IntoTokens, ToTokenTree, ToTokens, quote, quote_token};

#[test]
fn quote_macro() {
    let ts: typed_quote::tokens::Concat<
        typed_quote::tokens::punct::Sub<typed_quote::maybe_span::NoSpan>,
        typed_quote::tokens::ConstLiteral<_>,
    > = quote! {- 1};

    assert_eq!(ts.into_token_stream2().into_iter().count(), 2);
    assert_eq!(ts.into_token_stream2().to_string(), "- 1");

    let ts: typed_quote::tokens::Concat<
        typed_quote::tokens::punct::Sub<typed_quote::maybe_span::NoSpan>,
        typed_quote::tokens::ConstLiteral<_>,
    > = quote!(-1);

    assert_eq!(ts.into_token_stream2().into_iter().count(), 2);
    assert_eq!(ts.into_token_stream2().to_string(), "- 1");

    macro_rules! expect_one_tt {
        ($only:tt) => {};
    }

    macro_rules! quote_literal {
        ($lit:literal) => {{
            expect_one_tt! {$lit}
            quote!($lit)
        }};
    }

    let tt: typed_quote::tokens::ConstLiteral<_> = quote_literal!(-1);

    assert_eq!(tt.to_token_tree2().to_string(), "-1");
    assert_eq!(tt.to_token_stream2().into_iter().count(), 2);
    assert_eq!(tt.to_token_stream2().to_string(), "- 1");
}

#[test]
fn quote_token_macro() {
    let tt: typed_quote::tokens::ConstLiteral<_> = quote_token!(-1);
    assert_eq!(tt.to_token_tree2().to_string(), "-1");
    assert_eq!(tt.to_token_stream2().into_iter().count(), 2);
    assert_eq!(tt.to_token_stream2().to_string(), "- 1");

    let tt: typed_quote::tokens::punct::Sub<typed_quote::maybe_span::NoSpan> = quote_token!(-);
    assert_eq!(tt.to_token_tree2().to_string(), "-");
    assert_eq!(tt.to_token_stream2().into_iter().count(), 1);
    assert_eq!(tt.to_token_stream2().to_string(), "-");
}
