use proc_macro::{Group, Literal, Span, TokenStream, TokenTree};

use super::tokens;

pub trait WithSpan {
    type WithDefaultSpan: WithSpan<WithDefaultSpan = Self::WithDefaultSpan, WithReplacedSpan = Self::WithReplacedSpan>;
    fn with_default_span(self, span: Span) -> Self::WithDefaultSpan;

    type WithReplacedSpan: WithSpan<
            WithDefaultSpan = Self::WithReplacedSpan,
            WithReplacedSpan = Self::WithReplacedSpan,
        >;
    fn with_replaced_span(self, span: Span) -> Self::WithReplacedSpan;
}

pub trait HasSpan {
    fn replace_span(&mut self, span: Span);
}

pub trait ImplWithSpanForHasSpan: HasSpan {}

impl<T: ImplWithSpanForHasSpan> WithSpan for T {
    type WithDefaultSpan = Self;

    fn with_default_span(self, _: Span) -> Self::WithDefaultSpan {
        self
    }

    type WithReplacedSpan = Self;

    fn with_replaced_span(mut self, span: Span) -> Self::WithReplacedSpan {
        self.replace_span(span);

        self
    }
}

pub trait IntoTokenTree: WithSpan {
    fn into_token_tree(self) -> TokenTree;
}

pub trait IntoTokenStream: WithSpan {
    fn into_token_stream(self) -> TokenStream;
}

impl<T: IntoTokenTree> IntoTokenStream for T {
    fn into_token_stream(self) -> TokenStream {
        self.into_token_tree().into()
    }
}

impl WithSpan for tokens::Empty {
    type WithDefaultSpan = Self;

    fn with_default_span(self, _: Span) -> Self::WithDefaultSpan {
        self
    }

    type WithReplacedSpan = Self;

    fn with_replaced_span(self, _: Span) -> Self::WithReplacedSpan {
        self
    }
}
impl IntoTokenStream for tokens::Empty {
    fn into_token_stream(self) -> TokenStream {
        TokenStream::new()
    }
}

impl<T: IntoTokenStream> IntoTokenStream for Option<T> {
    fn into_token_stream(self) -> TokenStream {
        self.map(T::into_token_stream)
            .unwrap_or_else(TokenStream::new)
    }
}

impl<T: WithSpan> WithSpan for Option<T> {
    type WithDefaultSpan = Option<T::WithDefaultSpan>;

    fn with_default_span(self, span: Span) -> Self::WithDefaultSpan {
        self.map(|this| this.with_default_span(span))
    }

    type WithReplacedSpan = Option<T::WithReplacedSpan>;

    fn with_replaced_span(self, span: Span) -> Self::WithReplacedSpan {
        self.map(|this| this.with_replaced_span(span))
    }
}
impl<T: HasSpan> HasSpan for Option<T> {
    fn replace_span(&mut self, span: Span) {
        if let Some(this) = self {
            this.replace_span(span);
        }
    }
}

impl<I: IntoIterator<Item: IntoTokenStream>> IntoTokenStream for tokens::IterTokenStream<I> {
    fn into_token_stream(self) -> TokenStream {
        self.0
            .into_iter()
            .map(<I::Item as IntoTokenStream>::into_token_stream)
            .collect()
    }
}

impl<I> WithSpan for tokens::IterTokenStream<I> {
    type WithDefaultSpan = tokens::IterTokenStreamWithDefaultSpan<I, Span>;

    fn with_default_span(self, span: Span) -> Self::WithDefaultSpan {
        tokens::IterTokenStreamWithDefaultSpan(self.0, span)
    }

    type WithReplacedSpan = tokens::IterTokenStreamWithReplacedSpan<I, Span>;

    fn with_replaced_span(self, span: Span) -> Self::WithReplacedSpan {
        tokens::IterTokenStreamWithReplacedSpan(self.0, span)
    }
}

impl<I> WithSpan for tokens::IterTokenStreamWithDefaultSpan<I, Span> {
    type WithDefaultSpan = Self;

    fn with_default_span(self, _: Span) -> Self::WithDefaultSpan {
        self
    }

    type WithReplacedSpan = tokens::IterTokenStreamWithReplacedSpan<I, Span>;

    fn with_replaced_span(self, span: Span) -> Self::WithReplacedSpan {
        tokens::IterTokenStreamWithReplacedSpan(self.0, span)
    }
}

impl<I> ImplWithSpanForHasSpan for tokens::IterTokenStreamWithReplacedSpan<I, Span> {}
impl<I> HasSpan for tokens::IterTokenStreamWithReplacedSpan<I, Span> {
    fn replace_span(&mut self, span: Span) {
        self.1 = span;
    }
}

impl<A: IntoTokenStream, B: IntoTokenStream> IntoTokenStream for tokens::Concat<A, B> {
    fn into_token_stream(self) -> TokenStream {
        let Self(a, b) = self;
        let mut res = a.into_token_stream();
        res.extend(b.into_token_stream());
        res
    }
}

impl<A: WithSpan, B: WithSpan> WithSpan for tokens::Concat<A, B> {
    type WithDefaultSpan = tokens::Concat<A::WithDefaultSpan, B::WithDefaultSpan>;

    fn with_default_span(self, span: Span) -> Self::WithDefaultSpan {
        let Self(a, b) = self;
        tokens::Concat(a.with_default_span(span), b.with_default_span(span))
    }

    type WithReplacedSpan = tokens::Concat<A::WithReplacedSpan, B::WithReplacedSpan>;

    fn with_replaced_span(self, span: Span) -> Self::WithReplacedSpan {
        let Self(a, b) = self;
        tokens::Concat(a.with_replaced_span(span), b.with_replaced_span(span))
    }
}

impl<A: HasSpan, B: HasSpan> HasSpan for tokens::Concat<A, B> {
    fn replace_span(&mut self, span: Span) {
        let Self(a, b) = self;
        a.replace_span(span);
        b.replace_span(span);
    }
}

impl ImplWithSpanForHasSpan for Literal {}
impl HasSpan for Literal {
    fn replace_span(&mut self, span: Span) {
        self.set_span(span);
    }
}

impl IntoTokenTree for Literal {
    fn into_token_tree(self) -> TokenTree {
        TokenTree::from(self)
    }
}

pub struct WithReplacedSpan<T>(pub T, pub Span);

impl<T> ImplWithSpanForHasSpan for WithReplacedSpan<T> {}
impl<T> HasSpan for WithReplacedSpan<T> {
    fn replace_span(&mut self, span: Span) {
        self.1 = span;
    }
}

impl WithSpan for TokenStream {
    type WithDefaultSpan = Self;

    fn with_default_span(self, _: Span) -> Self::WithDefaultSpan {
        self
    }

    type WithReplacedSpan = WithReplacedSpan<Self>;

    fn with_replaced_span(self, span: Span) -> Self::WithReplacedSpan {
        WithReplacedSpan(self, span)
    }
}

impl IntoTokenStream for TokenStream {
    fn into_token_stream(self) -> TokenStream {
        self
    }
}

impl IntoTokenStream for WithReplacedSpan<TokenStream> {
    fn into_token_stream(self) -> TokenStream {
        let Self(ts, span) = self;

        ts.into_iter()
            .map(|tt| tt.with_replaced_span(span).into_token_tree())
            .collect()
    }
}

impl WithSpan for TokenTree {
    type WithDefaultSpan = Self;

    fn with_default_span(self, _: Span) -> Self::WithDefaultSpan {
        self
    }

    type WithReplacedSpan = WithReplacedSpan<Self>;

    fn with_replaced_span(self, span: Span) -> Self::WithReplacedSpan {
        WithReplacedSpan(self, span)
    }
}

impl IntoTokenTree for WithReplacedSpan<TokenTree> {
    fn into_token_tree(self) -> TokenTree {
        let Self(mut tt, span) = self;
        match &mut tt {
            TokenTree::Group(group) => {
                let mut new_group = Group::new(
                    group.delimiter(),
                    group.stream().with_replaced_span(span).into_token_stream(),
                );
                new_group.set_span(span);
                *group = new_group;
            }
            TokenTree::Ident(this) => this.set_span(span),
            TokenTree::Punct(this) => this.set_span(span),
            TokenTree::Literal(this) => this.set_span(span),
        }

        tt
    }
}

impl WithSpan for &TokenStream {
    type WithDefaultSpan = Self;

    fn with_default_span(self, _: Span) -> Self::WithDefaultSpan {
        self
    }

    type WithReplacedSpan = WithReplacedSpan<Self>;

    fn with_replaced_span(self, span: Span) -> Self::WithReplacedSpan {
        WithReplacedSpan(self, span)
    }
}

impl IntoTokenStream for &TokenStream {
    fn into_token_stream(self) -> TokenStream {
        self.clone()
    }
}

impl IntoTokenStream for WithReplacedSpan<&TokenStream> {
    fn into_token_stream(self) -> TokenStream {
        let Self(ts, span) = self;

        WithReplacedSpan(ts.clone(), span).into_token_stream()
    }
}

macro_rules! impl_group {
    ($($Ty:ident $WithSpan:ident)+) => {$(
        impl<Inner: IntoTokenStream> IntoTokenTree for tokens::$Ty<Inner> {
            fn into_token_tree(self) -> proc_macro::TokenTree {
                proc_macro::TokenTree::Group(proc_macro::Group::new(
                    proc_macro::Delimiter::$Ty,
                    self.0.into_token_stream(),
                ))
            }
        }
        impl<Inner: WithSpan> WithSpan for tokens::$Ty<Inner> {
            type WithDefaultSpan = tokens::$WithSpan<Inner::WithDefaultSpan, Span>;

            fn with_default_span(self, span: Span) -> Self::WithDefaultSpan {
                let Self(inner) = self;
                tokens::$WithSpan(inner.with_default_span(span), span)
            }

            type WithReplacedSpan = tokens::$WithSpan<Inner::WithReplacedSpan, Span>;

            fn with_replaced_span(self, span: Span) -> Self::WithReplacedSpan {
                let Self(inner) = self;
                tokens::$WithSpan(inner.with_replaced_span(span), span)
            }
        }

        impl<Inner: IntoTokenStream> IntoTokenTree for tokens::$WithSpan<Inner, Span> {
            fn into_token_tree(self) -> proc_macro::TokenTree {
                let Self(inner, span) = self;
                let mut group = tokens::$Ty(inner).into_token_tree();
                group.set_span(span);
                group
            }
        }

        impl<Inner: WithSpan> WithSpan for tokens::$WithSpan<Inner, Span> {
            type WithDefaultSpan = tokens::$WithSpan<Inner::WithDefaultSpan, Span>;

            fn with_default_span(self, span: Span) -> Self::WithDefaultSpan {
                let Self(inner, _) = self;
                tokens::$WithSpan(inner.with_default_span(span), span)
            }

            type WithReplacedSpan = tokens::$WithSpan<Inner::WithReplacedSpan, Span>;

            fn with_replaced_span(self, span: Span) -> Self::WithReplacedSpan {
                let Self(inner, _) = self;
                tokens::$WithSpan(inner.with_replaced_span(span), span)
            }
        }
        impl<Inner: HasSpan> HasSpan for tokens::$WithSpan<Inner, Span> {
            fn replace_span(&mut self, span: Span) {
                self.0.replace_span(span);
                self.1 = span;
            }
        }
    )+};
}

impl_group! {
    Parenthesis ParenthesisWithSpan
    Bracket     BracketWithSpan
    Brace       BraceWithSpan
}

// not spanned
impl_many!(
    impl<__> WithSpan
        for each_of![
            //
            tokens::Ident,
            tokens::token::Underscore,
            tokens::token::Colon2,
        ]
    {
        type WithDefaultSpan = Self::WithReplacedSpan;

        fn with_default_span(self, span: Span) -> Self::WithDefaultSpan {
            self.with_replaced_span(span)
        }

        type WithReplacedSpan = WithReplacedSpan<Self>;

        fn with_replaced_span(self, span: Span) -> Self::WithReplacedSpan {
            WithReplacedSpan(self, span)
        }
    }
);

impl IntoTokenTree for WithReplacedSpan<tokens::Ident> {
    fn into_token_tree(self) -> TokenTree {
        TokenTree::Ident(proc_macro::Ident::new(self.0.0, self.1))
    }
}

impl IntoTokenTree for WithReplacedSpan<tokens::token::Underscore> {
    fn into_token_tree(self) -> TokenTree {
        let Self(tokens::token::Underscore, span) = self;
        TokenTree::Ident(proc_macro::Ident::new("_", span))
    }
}

impl IntoTokenStream for WithReplacedSpan<tokens::token::Colon2> {
    fn into_token_stream(self) -> TokenStream {
        use proc_macro::Spacing::{Alone, Joint};

        let Self(tokens::token::Colon2, span) = self;

        TokenStream::from_iter([Joint, Alone].into_iter().map(|spacing| {
            TokenTree::Punct({
                let mut tt = proc_macro::Punct::new(':', spacing);
                tt.set_span(span);

                tt
            })
        }))
    }
}
