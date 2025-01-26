/// Module to run the live QA tests.
///
use crate::liveqa::liveassert::Result;

#[cfg(feature = "liveqa_tests")]
use inventory;

/// A simple descriptor for one “live test”.
pub struct LiveTest {
    pub name: &'static str,
    pub test_fn: fn() -> Result,
}

// Tell `inventory` to gather all `LiveTest` submissions in one registry.
#[cfg(feature = "liveqa_tests")]
inventory::collect!(LiveTest);

/// This macro behaves like a little attribute. It defines a function
/// returning `crate::liveqa::liveassert::Result`, and automatically registers it.
///
/// Usage:
/// ```
///
///live_test! {
///    fn mytest1()
///    {
///        test_assert!(1 == 0, "Expected 1 == 0, but they aren't equal");
///    }
///}
///
///live_test! {
///    fn mytest2()
///    {
///        test_assert!(1 == 0);
///    }
///}
/// ```
#[cfg(feature = "liveqa_tests")]
#[macro_export]
macro_rules! live_test {
    (fn $fn_name:ident() $body:block) => {
        fn $fn_name() -> crate::liveqa::liveassert::Result {
            // Wrap the user body in a closure so we can “append” Ok(()) automatically.
            (|| -> crate::liveqa::liveassert::Result {
                $body
                Ok(())
            })()
        }

        ::inventory::submit! {
            $crate::liveqa::runner::LiveTest {
                name: stringify!($fn_name),
                test_fn: $fn_name,
            }
        }
    };
}

/// This macro behaves like a little attribute. It defines a function
/// returning `crate::liveqa::liveassert::Result`, and automatically registers it.
///
/// Usage:
/// ```
/// live_test! {
///     fn mytest() -> crate::liveqa::liveassert::Result {
///         // do checks
///         if 1 == 0 {
///             return Err("1 was not 0!".to_string());
///         }
///         Ok(())
///     }
/// }
/// ```
#[cfg(not(feature = "liveqa_tests"))]
#[allow(unused_macros)]
macro_rules! live_test {
    (
       fn $fn_name:ident() -> crate::liveqa::liveassert::Result $body:block
    ) => {
        // Do nothing
    };
}

/// A helper to run *all* collected live tests and print results.
///
/// You could export this from the DLL and call it from the host
/// application whenever you like.
#[cfg(feature = "liveqa_tests")]
pub fn run_live_qa_tests() {
    let mut error_count: usize = 0;
    log::info!(
        "Running {} tests",
        inventory::iter::<LiveTest>.into_iter().count()
    );
    for test in inventory::iter::<LiveTest> {
        let outcome = (test.test_fn)();
        match outcome {
            Ok(()) => {
                log::info!("LIVE TEST PASSED: {}", test.name);
            }
            Err(e) => {
                log::error!("LIVE TEST FAILED: {}\n    {}", test.name, e);
                error_count += 1;
                // Maybe track how many fails, log them, etc.
            }
        }
    }

    if error_count > 0 {
        log::error!("LiveQA tests failed. {error_count} tests failed!");
        log::error!("The modder will not work. Shutting down.");
        // Swkotor will make the panic disappear, but it will halt executions
        panic!("Tests failed");
    }
}

/// A helper to run *all* collected live tests and print results.
///
/// You could export this from the DLL and call it from the host
/// application whenever you like.
///
/// Does nothing as `liveqa_tests` is not enabled
#[cfg(not(feature = "liveqa_tests"))]
pub fn run_live_qa_tests() {}
