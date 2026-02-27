pub use core::stringify;

use super::{Ident, Lifetime, NoSpan};

/// `ident` must be `stringify!($ident)` where `$ident:ident`
pub const fn ident(ident: &'static str) -> Ident<'static> {
    Ident(ident, NoSpan)
}

/// `lifetime` must be `stringify!($lifetime)` where `$lifetime:lifetime`
pub const fn lifetime(lifetime: &'static str) -> Lifetime<'static> {
    Lifetime(lifetime, NoSpan)
}
