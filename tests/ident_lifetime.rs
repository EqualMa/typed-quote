#![cfg(feature = "proc-macro2")]

use typed_quote::{IntoTokenTree, IntoTokens, quote};

#[test]
fn ident() {
    let a = quote!(my_ident);
    assert!(matches!(
        a.into_token_tree2(),
        proc_macro2::TokenTree::Ident(ident) if ident == "my_ident"
    ));
    assert_eq!(a.into_token_stream2().to_string(), "my_ident");

    let a = quote!(super);
    assert!(matches!(
        a.into_token_tree2(),
        proc_macro2::TokenTree::Ident(ident) if ident == "super"
    ));
    assert_eq!(a.into_token_stream2().to_string(), "super");

    let a = quote!(r#fn);
    assert!(matches!(
        a.into_token_tree2(),
        proc_macro2::TokenTree::Ident(ident) if ident == "r#fn"
    ));
    assert_eq!(a.into_token_stream2().to_string(), "r#fn");

    let a = quote!(_);
    assert!(matches!(
        a.into_token_tree2(),
        proc_macro2::TokenTree::Ident(ident) if ident == "_"
    ));
    assert_eq!(a.into_token_stream2().to_string(), "_");
}

#[test]
fn lifetime() {
    let a = quote!('my_ident);
    assert_eq!(a.into_token_stream2().to_string(), "'my_ident");

    let a = quote!('super);
    assert_eq!(a.into_token_stream2().to_string(), "'super");

    let a = quote!('r#fn);
    assert_eq!(a.into_token_stream2().to_string(), "'r#fn");

    let a = quote!('_);
    assert_eq!(a.into_token_stream2().to_string(), "'_");

    let a = quote!('static);
    assert_eq!(a.into_token_stream2().to_string(), "'static");
}
