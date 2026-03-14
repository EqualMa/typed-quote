use std::str::FromStr;

use typed_quote::{ToTokenTree, ToTokens, quote, quote_token};

macro_rules! test_quote {
    (#$quoted:ident) => {
        $quoted
    };
    (#$quoted:ident $($rest:tt)+) => {
        $crate::tokens::Concat(
            $quoted,
            $crate::quote!($($rest)+),
        )
    };
    () => {
        ()
    };
    ($only:tt) => {
        stringify!($only)
    };
    ($head:tt $($tail:tt)+) => {
        (stringify!($head), test_quote!($($tail)+))
    };
}

macro_rules! expect_one_tt {
    ($only:tt) => {};
}

macro_rules! quote_literal {
    ($lit:literal) => {{
        expect_one_tt! {$lit}
        quote!($lit)
    }};
}

/// ```
/// macro_rules! test_literal_from_macro { ($lit:literal) => {
///     ::test_proc_macro::test_literal! { $lit }
/// } }
/// test_literal_from_macro! { -1 }
/// ```
///
#[proc_macro]
pub fn test_literal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();

    if let [proc_macro::TokenTree::Group(g)] = input.as_slice()
        && g.delimiter() == proc_macro::Delimiter::None
        && let [
            proc_macro::TokenTree::Punct(sub),
            proc_macro::TokenTree::Literal(lit),
        ] = g.stream().into_iter().collect::<Vec<_>>().as_slice()
        && *sub == '-'
        && sub.spacing() == proc_macro::Spacing::Alone
        && lit.to_string() == "1"
    {
    } else {
        panic!("unexpected input: {:?}", input);
    }

    assert_eq!(
        proc_macro::Literal::from_str("-1").unwrap().to_string(),
        "-1"
    );

    assert_eq!(test_quote!(-1), ("-", "1"));

    assert_eq!(quote!(-1).to_token_stream().to_string(), "- 1");

    assert_eq!(quote_token!(-1).to_token_tree().to_string(), "-1");
    assert_eq!(quote_token!(-1).to_token_stream().to_string(), "-1");

    let ts: typed_quote::tokens::ConstLiteral<_> = quote_literal!(-1);

    assert_eq!(ts.to_token_tree().to_string(), "-1");
    assert_eq!(ts.to_token_stream().to_string(), "-1");

    Default::default()
}
