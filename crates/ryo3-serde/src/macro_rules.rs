#[macro_export]
macro_rules! serde_err {
    ($($arg:tt)*) => {
        Err(::serde::ser::Error::custom(format_args!($($arg)*)))
    }
}

#[macro_export]
macro_rules! serde_err_recursion {
    () => {
        Err(::serde::ser::Error::custom("recursion"))
    };
}
