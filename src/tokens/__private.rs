#[doc(hidden)]
pub use core::stringify;

use super::{Ident, Lifetime, Literal, NoSpan};

/// `ident` must be `stringify!($ident)` where `$ident:ident`
#[doc(hidden)]
pub const fn ident(ident: &'static str) -> Ident<'static> {
    Ident(ident, NoSpan)
}

/// `lifetime` must be `stringify!($lifetime)` where `$lifetime:lifetime`
#[doc(hidden)]
pub const fn lifetime(lifetime: &'static str) -> Lifetime<'static> {
    Lifetime(lifetime, NoSpan)
}

/// `literal` must be `stringify!($lit)` where `$lit:literal`
#[doc(hidden)]
pub const fn literal(literal: &'static str) -> Literal<'static> {
    Literal(literal, NoSpan)
}
