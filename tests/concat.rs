#![cfg(any(feature = "proc-macro", feature = "proc-macro2"))]

use typed_quote::{IntoTokens, WithSpan, quote};

#[cfg(feature = "proc-macro")]
extern crate proc_macro;

#[cfg(feature = "proc-macro")]
fn compile_only() {
    let lit = proc_macro::Literal::string("hello");
    let ts = quote!(compile_error! { #lit }).into_token_stream();

    let _ = ts.with_replaced_span(proc_macro::Span::call_site());

    const _: fn() = compile_only;
}

#[cfg(feature = "proc-macro2")]
#[test]
fn concat() {
    let lit = &proc_macro2::Literal::string("hello");
    let ts = quote!(compile_error! { #lit }).into_token_stream2();
    assert_eq!(ts.to_string(), "compile_error ! { \"hello\" }");

    let ts = quote!(compile_error! { #lit })
        .with_default_span(proc_macro2::Span::call_site())
        .into_token_stream2();
    assert_eq!(ts.to_string(), "compile_error ! { \"hello\" }");
}
