#![cfg(feature = "proc-macro2")]
#![cfg(feature = "alloc")]

use std::rc::Rc;

use proc_macro2::TokenStream;
use typed_quote::{IntoTokens, ToTokens, quote};

fn into_tokens(into: Box<dyn '_ + IntoTokens>) -> TokenStream {
    into.into_token_stream2()
}

fn to_tokens(to: &dyn ToTokens) -> TokenStream {
    to.to_token_stream2()
}

#[test]
fn r#dyn() {
    assert_eq!(
        into_tokens(Box::new(quote!(dyn compatible))).to_string(),
        "dyn compatible",
    );
    assert_eq!(
        to_tokens(&quote!(&dyn compatible)).to_string(),
        "& dyn compatible",
    );
    assert_eq!(
        to_tokens(&Box::new(quote!(&dyn compatible))).to_string(),
        "& dyn compatible",
    );
    assert_eq!(
        to_tokens(&Rc::new(quote!(&dyn compatible))).to_string(),
        "& dyn compatible",
    );
}
