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
