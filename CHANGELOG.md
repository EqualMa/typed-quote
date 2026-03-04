# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/EqualMa/typed-quote/releases/tag/v0.1.0) - 2026-03-04

### Added

- as_ident, as_literal, as_lifetime
- impl IntoTokenTree, ToTokenTree for Box, Rc
- impl RefWithSpan for IterTokens
- `quote!(ident) quote!('lifetime) quote("literal")` are now const expr
- prelude typed_quote
- impl Default for ConstIdent, ConstLifetime, ConstLiteral
- impl IntoTokens and ToTokens for Box and Rc
- [**breaking**] `alloc` is now a default cargo feature
- literal
- punct `$` and `~`
- prelude

### Fixed

- clippy warnings
- Ident("$crate") should be protected from span replacing
- punct(s) in quote_token!
- impl (Ref)WithSpan for group

### Other

- release_always = false
- setup release-plz
- add badges
- ci
- specify msrv
- vscode tasks
- no_std for cheat sheet doc tests
- doc cheat sheet
- docs
- vscode rust-analyzer
- metadata
- test concat
- fix features
- proc_macro and proc_macro2
- Never, Option, Either
- ident and lifetime
- punct
- puncts
- redesign group
- tokens::IterTokens*
- redesign traits
- wip
