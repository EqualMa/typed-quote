#[allow(unused)]
struct TestOk;

macro_rules! assert_underscore_not_ident {
    ($t:ident) => {
        compile_error!(stringify!($t))
    };
    ($t:tt) => {
        TestOk
    };
}

const _: TestOk = assert_underscore_not_ident!(_);

macro_rules! dollar_crate {
    () => {
        expect_one_ident! {$crate}
    };
}

macro_rules! expect_one_ident {
    ($ident:ident) => {
        TestOk
    };
}

const _: TestOk = dollar_crate!();
