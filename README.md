# typed-quote

[<img alt="github" src="https://img.shields.io/github/stars/EqualMa/typed-quote?style=for-the-badge&color=7AADE1&logo=github" height="20">](https://github.com/EqualMa/typed-quote)
[<img alt="docs.rs" src="https://img.shields.io/crates/v/typed-quote?style=for-the-badge&logo=rust" height="20">](https://crates.io/crates/typed-quote)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/typed-quote?style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/typed-quote)
[<img alt="Crates.io MSRV" src="https://img.shields.io/crates/msrv/typed-quote?style=for-the-badge" height="20">](https://github.com/EqualMa/typed-quote/blob/main/Cargo.toml#L10)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/EqualMa/typed-quote/ci.yml?style=for-the-badge" height="20">](https://github.com/EqualMa/typed-quote/actions)

A fully typed [quote!()](https://docs.rs/quote) alternative
for both [proc-macro](https://doc.rust-lang.org/stable/proc_macro/)
and [proc-macro2](https://docs.rs/proc-macro2).

## Quick start

### Output TokenStream

`typed_quote::quote!(...)` returns a fully typed value that implements
[`IntoTokens`], [`ToTokens`], [`WithSpan`], [`IntoTokenTree`] or [`ToTokenTree`]
depending on the quoted content.

- Call [ts.into_token_stream()](IntoTokens::into_token_stream) to get [`proc_macro::TokenStream`].
- Call [ts.into_token_stream2()](IntoTokens::into_token_stream2) to get [`proc_macro2::TokenStream`].

```rust
use typed_quote::prelude::*;

let crate_name = quote!("typed-quote");
let tokens = quote!(hello #crate_name !);

let ts: proc_macro2::TokenStream = tokens.into_token_stream2();

assert_eq!(ts.to_string(), r#"hello "typed-quote" !"#);
```

### Set span

- Call [`ts.with_default_span(span)`](WithSpan::with_default_span) to
  specify a span only for un-spanned tokens in `ts`.
  - `quote!(un-spanned).with_default_span(span)` will specify span for all tokens.
  - `quote!(#quoted_var).with_default_span(span)` will call `quoted_var.with_default_span(span)`.
  - `proc_macro::TokenStream::from_str("...").unwrap().with_default_span(span)` will not change span because
    `proc_macro::TokenStream` already has a span.

- Call [`ts.with_replaced_span(span)`](WithSpan::with_replaced_span) to set span for all tokens in `ts`.
  - Note that, if the ident of [`proc_macro::Ident`] and [`proc_macro2::Ident`] is exactly `$crate`,
    then its span will not be replaced.

### Comparison with [`::quote`](https://docs.rs/quote)

`typed_quote::typed_quote!()` is an alias of `typed_quote::quote!()`
so you can disambiguate `typed_quote!()` from `quote::quote!()`.

- `typed_quote` is new, not well tested, and will contain breaking changes in the future.
  You should use `::quote` for production.
- [`::quote::quote!()`](https://docs.rs/quote/latest/quote/macro.quote.html)
  returns a `proc_macro2::TokenStream`.

  `typed_quote!()` returns a fully typed struct that implements `IntoTokens`
  and you can decide to output `proc_macro::TokenStream` or `proc_macro2::TokenStream`.

- `::quote` has `quote_spanned!(span => ...)`.

  But in `typed_quote`, use `typed_quote!(...).with_default_span(span)` instead.

- `::quote::quote!(#var)` only references `&var` but `typed_quote!(#var)` moves `var`.
  Since `&impl ToTokens` implements both `IntoToken` and `Copy`,
  `let var = &var; typed_quote!(#var)` will work if `var` is a value of `impl ToTokens`.

- `::quote::ToTokens` is dyn-

## cargo features

- `alloc` _(default)_

  This feature implements traits for `Box`, `Rc`.

  This library is no_std and only `alloc` is enabled by default.

- `proc-macro` and `proc-macro2`

  Enable one or both of these features if necessary:

  ```sh
  cargo add typed-quote --features proc-macro
  cargo add typed-quote --features proc-macro2
  cargo add typed-quote --features proc-macro,proc-macro2
  ```

  You don't need to enable them when you're writing a library that just inputs/outputs tokens.
  Users of your library can decide which feature to enable and call
  the corresponding `into_token_stream` or `into_token_stream2`.

  ```sh
  cargo add typed-quote
  ```

  ```rust
  use typed_quote::prelude::*;

  fn braced(stream: impl IntoTokens) -> impl IntoTokenTree {
      quote!({ stream })
  }
  ```
