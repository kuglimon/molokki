/// Module to provide assertions and mechanisms to handle for the live QA tests.
///

pub(crate) type Result = std::result::Result<(), Box<dyn std::error::Error>>;

/// A macro that behaves like `assert!` but returns an Err(...) instead of panicking.
#[macro_export]
macro_rules! test_assert {
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            return Err(format!(
                "{} at {}:{}", $($arg)*, file!(), line!()
            ).into());
        }
    };
    ($cond:expr) => {
        if !$cond {
            return Err(format!(
                "Assertion failed: {} at {}:{}",
                stringify!($cond),
                file!(),
                line!()
            ).into());
        }
    };
}