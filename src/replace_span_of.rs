use crate::{IntoTokens, WithSpan};

pub trait ReplaceSpanOf<T: IntoTokens + WithSpan> {
    type ReplaceSpanOf: IntoTokens + WithSpan;

    fn replace_span_of(self, t: T) -> Self::ReplaceSpanOf;
}
