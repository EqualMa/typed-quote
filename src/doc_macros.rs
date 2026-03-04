macro_rules! doc_cheat_sheet {() => [ crate::doc_macros::doc_impl!(
    // Type(Traits)(example_values),
    Never(
        [Into/ToTokenTree],
        [Into/ToTokens],
        [(Ref)WithSpan],
    )(),
    {"Either<A, B>", A, B}(
        [Into/ToTokenTree],
        [Into/ToTokens],
        [(Ref)WithSpan],
    )("\
if condition {
    Either::A(quote!(a))
} else {
    Either::B(quote!(b))
}\
    "),
    {"Option<T>", T}(
        [Into/ToTokens],
        [(Ref)WithSpan],
    )("\
if condition {
    Some(quote!(some))
} else {
    None
}\
    "),
    (
        doc = "`&T`",
        code = {"&T", T},
    )(
        [Into/ToTokens] where [T: ToTokens],
        [(Ref)WithSpan] where [T: RefWithSpan],
    )("&quote!(ref)"),
    (
        doc = "[`Box<T: ?Sized>`](alloc::boxed::Box)",
        code = {"Box<T>", T: ?Sized},
    )(
        [Into/ToTokenTree],
        [Into/ToTokens],
    )(
        "Box::new(quote!(_))",
        "Box::new(quote!(_)) as Box<dyn ToTokenTree>",
    ),
    (
        doc = "[`Rc<T: ?Sized>`](alloc::rc::Rc)",
        code = {"Rc<T>", T: ?Sized},
    )(
        [Into/ToTokenTree] where [T: ToTokenTree],
        [Into/ToTokens] where [T: ToTokens],
    )(
        "Rc::new(quote!(_))",
        "Rc::new(quote!(_)) as Rc<dyn ToTokenTree>",
    ),
    {("Empty")}(
        [Into/ToTokens],
        [(Ref)WithSpan],
    )("quote!()"),
    [
        doc = code_not_transparent(
            "grouped token stream:\
            \n\n`{...}`\
            \n\n`[...]`\
            \n\n`(...)`\
            "
        ),
        {("Brace",       "<S>"), S},
        {("Bracket",     "<S>"), S},
        {("Parenthesis", "<S>"), S},
    ](
        [Into/ToTokens],
        [(Ref)WithSpan],
    )(
        "quote!( { braced tokens } )",
        "quote!( [ 0, 1, 2, 3, 4 ] )",
        "quote!( ( #tokens )       )",
    ),
    {const Ident as_ident}(
        [Into/ToTokenTree],
        [Into/ToTokens],
        [(Ref)WithSpan],
    )("quote!(my_ident)"),
    {const Lifetime as_lifetime}(
        [Into/ToTokens],
        [(Ref)WithSpan],
    )("quote!('my_lifetime)"),
    {const Literal as_literal}(
        [Into/ToTokenTree],
        [Into/ToTokens],
        [(Ref)WithSpan],
    )(r#"quote!("my literal")"#),
    {("Concat", "<A, B>"), A, B}(
        [Into/ToTokens],
        [(Ref)WithSpan],
    )("let a = quote!(a);\n\
        let b = quote!(b);\n\
        quote!(#a #b)"),
    (
        doc = "[`IterTokens<I: IntoIterator>`]\
            (crate::tokens::IterTokens)",
        code = {"IterTokens<I>", I: IntoIterator}
    )(
        [IntoTokens]  where ["I::Item": IntoTokens          ],
        [ToTokens]    where ["I::Item": IntoTokens, I: Clone],
        [WithSpan]    where ["I::Item": WithSpan            ],
        [RefWithSpan] where ["I::Item": WithSpan,   I: Clone],
    )(
        "IterTokens([quote!(a); 10])"
    ),
    (
        doc = "punctuation",
        code = {"T", T}
    )(
        [Into/ToTokenTree],
        [Into/ToTokens],
        [(Ref)WithSpan],
    )(
        "quote!( : )",
        "quote!( . )",
        "quote!( * )",
    ),
    (
        doc = "punctuations",
        code = {"T", T}
    )(
        [Into/ToTokens],
        [(Ref)WithSpan],
    )(
        "quote!( :: )",
        "quote!( .. )",
        "quote!( *= )",
    ),
    (proc_macro::{
        TokenTree,
        Group,
        Ident,
        Punct,
        Literal,
    })(
        [Into/ToTokenTree],
        [Into/ToTokens],
        [(Ref)WithSpan],
    )(r#"proc_macro::Ident::new(
    "tt",
    proc_macro::Span::call_site()
)"#),
    (proc_macro::TokenStream)(
        [Into/ToTokens],
        [(Ref)WithSpan],
    )(r#"proc_macro::TokenStream::new()"#),
) ]}

macro_rules! code_transparent {
    ($($inner:expr),* $(,)?) => {
        concat!(
            " class='inner-code-bg-transparent'>\n\n",
            $($inner,)*
            "\n",
        )
    };
}

macro_rules! td_type {
    ($Type:ident) => {
        crate::doc_macros::td_type!({ stringify!($Type) })
    };
    ({const $Type:ident $as_val:ident}) => {
        crate::doc_macros::code_transparent!(
            "[`Const",
            stringify!($Type),
            "<T: ?Sized + HasConst",
            stringify!($Type),
            ">`](crate::tokens::Const",
            stringify!($Type),
            ")",
            //
            "\n\n",
            //
            "[`",
            stringify!($Type),
            "<'_>`](crate::tokens::",
            stringify!($Type),
            ")",
        )
    };
    ({
        ($Type:expr $(, $Generics:expr)?)
        $(, $($rest:tt)*)?
    }) => {
        crate::doc_macros::code_transparent!(
            "[`",
            $Type,
            $($Generics,)?
            "`](crate::tokens::",
            $Type,
            ")",
        )
    };
    ({
        $doc:expr
        $(, $($rest:tt)*)?
    }) => {
        crate::doc_macros::code_transparent!(
            "[`",
            $doc,
            "`]",
        )
    };
    ([
        doc = $($rest:tt)*
    ]) => {
        crate::doc_macros::td_type!( (doc = $($rest)*) )
    };
    ((
        doc = code_not_transparent($doc:expr)
        $(, $($rest:tt)*)?
    )) => {
        concat!(
            ">\n\n",
            $doc,
            "\n",
        )
    };
    ((
        doc = $doc:expr
        $(, $($rest:tt)*)?
    )) => {
        crate::doc_macros::code_transparent!(
            $doc
        )
    };
    ((
        proc_macro::$proc_macro_import:ident
    )) => {
        concat!(
            ">\n\n<code style='background-color:transparent'>",
            "[proc_macro]\\([2](proc_macro2))::[",
            stringify!($proc_macro_import),
            "](proc_macro::",
            stringify!($proc_macro_import),
            ")\\([2](proc_macro2::",
            stringify!($proc_macro_import),
            "))</code>\n",
        )
    };
    ((
        proc_macro::{$($proc_macro_import:ident),+ $(,)?}
    )) => {
        concat!(
            ">\n\n<code style='background-color:transparent'>",
            "[proc_macro]\\([2](proc_macro2))::{",
            $(
                "\n&nbsp;&nbsp;[",
                stringify!($proc_macro_import),
                "](proc_macro::",
                stringify!($proc_macro_import),
                ")\\([2](proc_macro2::",
                stringify!($proc_macro_import),
                ")),",
            )+
            "\n}</code>\n",
        )
    };
}

macro_rules! td_trait_one_where {
    ([]) => {
        ""
    };
    ([$($Ty:tt : $Bound:ident),+]) => {
        concat!(
            " where "
            $(
                ,
                crate::doc_macros::literal_or_stringify!($Ty),
                ": [",
                stringify!($Bound),
                "]",
            )", "+
        )
    };
}

macro_rules! td_trait_one {
    ([Into/ToTokenTree] $(where $bounds:tt)?) => {
        crate::doc_macros::td_trait_one!(
            ("[Into](IntoTokenTree)/[ToTokenTree]")
            $(where $bounds)?
        )
    };
    ([Into/ToTokens] $(where $bounds:tt)?) => {
        crate::doc_macros::td_trait_one!(
            ("[Into](IntoTokens)/[ToTokens]")
            $(where $bounds)?
        )
    };
    ([(Ref)WithSpan] $(where $bounds:tt)?) => {
        crate::doc_macros::td_trait_one!(
            ("\\([Ref](RefWithSpan))[WithSpan]")
            $(where $bounds)?
        )
    };
    ([$($Trait:ident),+] $(where $bounds:tt)?) => {
        crate::doc_macros::td_trait_one!(
            ("" $(
                ,
                "[",
                stringify!($Trait),
                "]",
            )", "+)
            $(where $bounds)?
        )
    };
    (($($Trait:expr),* $(,)?) $(where $bounds:tt)?) => {
        concat!(
            "<code>",
            $($Trait,)*
            $(crate::doc_macros::td_trait_one_where!($bounds),)?
            "</code>",
        )
    };
}

macro_rules! td_trait {
    (($($t:tt $(where $bounds:tt)?),+ $(,)?)) => {
        concat!(
            ">",
            $(
                "\n\n",
                crate::doc_macros::td_trait_one!(
                    $t $(where $bounds)?
                ),
            )+
            "\n\n",
        )
    };
}

macro_rules! test_assert_impl {
    (
        s_ty $s_ty:tt
        generics $generics:tt
        traits $traits:tt
        s_values($($s_values:expr),* $(,)?)
    ) => {
        concat!($(
            crate::doc_macros::test_assert_impl_one_value!(
                s_ty $s_ty
                generics $generics
                traits $traits
                s_value ($s_values)
            ),
        )*)
    };
}

macro_rules! test_assert_impl_one_value {
    (
        s_ty $s_ty:tt
        generics $generics:tt
        traits(
            $($trait_group:tt $(where $bounds:tt)?),* $(,)?
        )
        s_value($value:expr)
    ) => {
        concat!(
            "```
# \
use std::rc::Rc;\
use proc_macro2 as proc_macro;\
use typed_quote::{prelude::*, tokens::*, WithSpan, RefWithSpan, Either};\
fn main() {\
let condition = true;\
let tokens = quote!();\
let get_value = || {
", $value, "
# };",
            $(crate::doc_macros::test_assert_impl_trait_group!(
                (
                    s_ty $s_ty
                    generics $generics
                    s_value($value)
                )
                $trait_group
                $(where $bounds)?
            ),)*
            "}\n```\n\n",
        )
    };
}

macro_rules! test_assert_impl_trait_group {
    ($common:tt [Into/ToTokenTree] $(where $bounds:tt)?) => {
        crate::doc_macros::test_assert_impl_trait_group!(
            $common
            [IntoTokenTree, ToTokenTree]
            $(where $bounds)?
        )
    };
    ($common:tt [Into/ToTokens] $(where $bounds:tt)?) => {
        crate::doc_macros::test_assert_impl_trait_group!(
            $common
            [IntoTokens, ToTokens]
            $(where $bounds)?
        )
    };
    ($common:tt [(Ref)WithSpan] $(where $bounds:tt)?) => {
        crate::doc_macros::test_assert_impl_trait_group!(
            $common
            [WithSpan, RefWithSpan]
            $(where $bounds)?
        )
    };
    ($common:tt [$($Trait:ident),+] $(where $bounds:tt)?) => {
        crate::doc_macros::test_assert_impl_trait_group!(
            $common
            ($(stringify!($Trait),)+)
            $(where $bounds)?
        )
    };
    ($common:tt ($($Trait:expr),* $(,)?)) => {
        concat!($(
            crate::doc_macros::test_assert_impl_one_trait!(
                $common
                s_trait($Trait)
                where()
            ),
        )*)
    };
    ($common:tt ($($Trait:expr),* $(,)?) where $bounds:tt) => {
        concat!($(
            crate::doc_macros::test_assert_impl_one_trait!(
                $common
                s_trait($Trait)
                where($bounds)
            ),
        )*)
    };
}

macro_rules! test_assert_impl_one_trait {
    (
        (
            s_ty($Ty:expr)
            generics $generics:tt
            s_value($value:expr)
        )
        s_trait($Trait:expr)
        where $where:tt
    ) => {
        concat!(
"{ fn assert_impl<",
    stringify! $generics,
">(v: ", $Ty, ") -> impl ", $Trait, " where ",
crate::doc_macros::test_assert_impl_one_trait_where!(
    generics $generics
    s_trait($Trait)
    where $where
),
"{v} \
_ = assert_impl(get_value());}",
// "\n# macro_rules! debug_tokens {() => {
// s_ty = ",$Ty, "
// generics = ", stringify! $generics, "
// s_trait = ", $Trait, "
// where = ", stringify! $where, "
// # }}",
        )
    };
}

macro_rules! test_assert_impl_one_trait_where {
    (
        generics($($TP:tt $(: $(?)? $TPBound:ident)?),* $(,)?)
        s_trait($Trait:expr)
        where()
    ) => {
        concat!(
            $(
                crate::doc_macros::literal_or_stringify!($TP),
                ":",
                $Trait,
                ",",
            )*
        )
    };
    (
        generics($($TP:tt $(: $(?)? $TPBound:ident)?),* $(,)?)
        s_trait($Trait:expr)
        where([$($Ty:tt : $Bound:ident),+ $(,)?])
    ) => {
        concat!(
            $(
                crate::doc_macros::literal_or_stringify!($Ty),
                ":",
                stringify!($Bound),
                ",",
            )+
        )
    };
}

macro_rules! td_value {
    (
        $type:tt
        $traits:tt
        // no examples
        ()
    ) => {
        ">"
    };
    (
        // code
        {const $Type:ident $as_val:ident}
        $traits:tt
        ($value:expr)
    ) => {
        crate::doc_macros::td_value!(
            [
                doc = _,
                {
                    (concat!(
                        "Const",
                        stringify!($Type),
                        "<impl ?Sized + HasConst",
                        stringify!($Type),
                        ">",
                    ))
                },
                {
                    (concat!(
                        stringify!($Type),
                        "<'_>",
                    ))
                },
            ]
            $traits
            (
                $value,
                concat!(
                    $value,
                    ".",
                    stringify!($as_val),
                    "()"
                )
            )
        )
    };
    (
        (proc_macro::$proc_macro_import:tt)
        $traits:tt
        $example_values:tt
    ) => {
        crate::doc_macros::td_value!(
            {"T", T}
            $traits
            $example_values
        )
    };
    (
        // code
        {$($code:tt)*}
        $traits:tt
        $example_values:tt
    ) => {
        concat!(
            " class='cell-full-of-code'>\n\n",
            crate::doc_macros::td_value_inner!(
                {$($code)*}
                $traits
                $example_values
            ),
            "\n\n",
        )
    };
    (
        (
            doc = $doc:expr,
            code = $code:tt
            $(, $($rest:tt)*)?
        )
        $traits:tt
        $example_values:tt
    ) => {
        crate::doc_macros::td_value!(
            $code
            $traits
            $example_values
        )
    };
    (
        [
            doc = $doc:expr
            $(, {$($code:tt)*})*
            $(,)?
        ]
        $traits:tt
        (
            $($value:expr),*
            $(,)?
        )
    ) => {
        concat!(
            " class='cell-full-of-code'>\n\n",
            $(
                crate::doc_macros::td_value_inner!(
                    {$($code)*}
                    $traits
                    ($value)
                ),
            )*
            "\n\n",
        )
    };
}

macro_rules! td_value_inner {
    (
        {$Ty:tt $(, $($Generics:tt)* )?}
        $traits:tt
        $example_values:tt
    ) => {
        crate::doc_macros::test_assert_impl!(
            s_ty(crate::doc_macros::literal_or_concat_paren!($Ty))
            generics($($($Generics)*)?)
            traits $traits
            s_values $example_values
        )
    };
}

macro_rules! doc_impl_row {
    (
        $type:tt
        $traits:tt
        $example_values:tt
    ) => {
        concat!(
            "<tr><td",
            crate::doc_macros::td_type!($type),
            "</td><td",
            crate::doc_macros::td_trait!($traits),
            "</td><td",
            crate::doc_macros::td_value!($type $traits $example_values),
            "</td></tr>"
        )
    };
}

macro_rules! doc_impl {
    (
        $($type:tt $traits:tt $example_values:tt),+ $(,)?
    ) => { concat!($(
        crate::doc_macros::doc_impl_row!(
            $type
            $traits
            $example_values
        ),
    )+)};
}

macro_rules! literal_or_stringify {
    ($v:literal) => {
        $v
    };
    ($id:ident) => {
        stringify!($id)
    };
}

macro_rules! literal_or_concat_paren {
    ($v:literal) => {
        $v
    };
    (($($e:expr),* $(,)?)) => {
        concat!($($e),*)
    };
}

pub(crate) use {
    code_transparent, doc_cheat_sheet, doc_impl, doc_impl_row, literal_or_concat_paren,
    literal_or_stringify, td_trait, td_trait_one, td_trait_one_where, td_type, td_value,
    td_value_inner, test_assert_impl, test_assert_impl_one_trait, test_assert_impl_one_trait_where,
    test_assert_impl_one_value, test_assert_impl_trait_group,
};
