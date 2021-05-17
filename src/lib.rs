/*
MIT License

Copyright (c) 2021 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! The crate `tokio-js-set-interval` allows you to use `setInterval(callback, ms)` and
//! `setTimeout(callback, ms)` as in Javascript inside a `tokio` runtime (https://tokio.rs/).
//! For this, it offers the macros `set_interval!(callback, ms)` and `set_timeout!(callback, ms)`.
//!
//! ## Restrictions
//! They behave similar to their Javascript counterparts, with a few exceptions:
//!
//!  * They only get executed if the `tokio` runtime lives long enough.
//!  * on order to compile, the callback must return the union type, i.e. `()`
//!  * => all actions must be done via side effects
//!  * ⚠ again, there is **NO GUARANTEE** that the tasks will get executed \
//!    (--> but useful/convenient for low priority background tasks and for the learning effect of course) ⚠
//!
//! ## Trivia
//! ⚠ I'm not an expert in `tokio` (or async/await/futures in Rust in general) and I don't
//!   know if this follows best practises. But it helped me to understand how `tokio` works.
//!   I hope it may be helpful to some of you too. ⚠
//!
//! The functionality itself is really simple. The biggest part are the convenient macros.
//! I over-engineered them a little to learn more about macros.
//!
//! ## Compatibility
//! Version 1.0.0 is developed with:
//!  * `tokio` @ 1.6.0 (but should also work with 1.0.0)
//!  * `rustc` @ 1.52.1 (but should also work with 1.45.2)

use tokio::time::Duration;

/// **INTERNAL** Use macro [`set_timeout`] instead!
///
/// Creates a future that glues the tokio sleep function (timeout) together with
/// the provided callback.
pub async fn _set_timeout(f: impl Fn(), ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    f();
}

/// **INTERNAL** Use macro [`set_interval`] instead!
///
/// Creates a future that glues the tokio interval function together with
/// the provided callback.
pub async fn _set_interval(f: impl Fn(), ms: u64) {
    let mut int = tokio::time::interval(Duration::from_millis(ms));

    // the first tick in tokios interval returns immediately. Because we want behaviour
    // similar to Javascript in this library, we work around this.
    int.tick().await;

    // this looks like it runs for ever, but this is not how tokio works
    // at least with tokio 1.6.0 and Rust 1.52 this stops executing
    // when the tokio runtime gets dropped.
    loop {
        int.tick().await;
        f();
    }
}

/// Creates a timeout that behaves similar to `setTimeout(callback, ms)` in Javascript
/// for the `tokio` runtime. Unlike in Javascript, it will only be executed, if after the
/// specified time passed, the `tokio` runtime still lives, i.e. didn't got dropped.
///
/// As in Javascript, a timeout may only have side effects and no return type.
/// You don't get a handle to manually wait for it, you must ensure, that the tokio
/// runtime lives long enough.
///
/// # Parameters
/// * `#1` expression, closure-expression, block or identifier (which points to a closure).
///        The code that represents the callback function.
/// * `#2` time delay in milliseconds
///
/// # Example
/// ```rust
/// use tokio::time::Duration;
/// use tokio_js_set_interval::set_timeout;
///
/// #[tokio::main]
/// async fn main() {
///     set_timeout!(println!("hello1"), 0);
///     println!("hello2");
///     // prevent that tokios runtime gets dropped too early
///     // order of output should be
///     //  "hello2"
///     //  "hello1"
///     tokio::time::sleep(Duration::from_millis(1)).await;
/// }
/// ```
#[macro_export]
macro_rules! set_timeout {
    // match for identifier, i.e. a closure, that is behind a variable
    ($cb:ident, $ms:literal) => {
        tokio::spawn($crate::_set_timeout($cb, $ms));
    };
    // match for direct closure expression
    (|| $cb:expr, $ms:literal) => {
        tokio::spawn($crate::_set_timeout(|| $cb, $ms));
    };
    // match for direct move closure expression
    (move || $cb:expr, $ms:literal) => {
        tokio::spawn($crate::_set_timeout(move || $cb, $ms));
    };
    // match for expr, like `set_timeout!(println!())`
    ($cb:expr, $ms:literal) => {
        tokio::spawn($crate::_set_timeout(|| $cb, $ms));
    };
    // match for block
    ($cb:block, $ms:literal) => {
        tokio::spawn($crate::_set_timeout(|| $cb, $ms));
    };
}

/// Creates a timeout that behaves similar to `setInterval(callback, ms)` in Javascript
/// for the `tokio` runtime. Unlike in Javascript, it will only be executed, if after the
/// specified time passed, the `tokio` runtime still lives, i.e. didn't got dropped.
///
/// As in Javascript, an interval may only have side effects and no return type.
/// You don't get a handle to manually wait for it, you must ensure, that the tokio
/// runtime lives long enough.
///
/// The interval is not stoppable (unless the application stops). Strictly speaking, there
/// should be a way to clear an interval, similar to javascript. But because this library is
/// mainly for educational reasons and for "low priority indefinetely running background tasks",
/// this is not implemented yet. If you need it, create an issue or a PR.
///
/// # Parameters
/// * `#1` expression, closure-expression, block or identifier (which points to a closure).
///        The code that represents the callback function.
/// * `#2` time delay in milliseconds
///
/// # Example
/// ```rust
/// use tokio::time::Duration;
/// use tokio_js_set_interval::set_interval;
///
/// #[tokio::main]
/// async fn main() {
///     set_interval!(println!("hello1"), 50);
///     println!("hello2");
///     // prevent that tokios runtime gets dropped too early
///     // "hello1" should get printed 2 times (50*2 == 100 < 120)
///     tokio::time::sleep(Duration::from_millis(120)).await;
/// }
/// ```
#[macro_export]
macro_rules! set_interval {
    // match for identifier, i.e. a closure, that is behind a variable
    ($cb:ident, $ms:literal) => {
        tokio::spawn($crate::_set_interval($cb, $ms));
    };
    // match for direct closure expression
    (|| $cb:expr, $ms:literal) => {
        tokio::spawn($crate::_set_interval(|| $cb, $ms));
    };
    // match for direct move closure expression
    (move || $cb:expr, $ms:literal) => {
        tokio::spawn($crate::_set_interval(move || $cb, $ms));
    };
    // match for expr, like `set_interval!(println!())`
    ($cb:expr, $ms:literal) => {
        tokio::spawn($crate::_set_interval(|| $cb, $ms));
    };
    // match for block
    ($cb:block, $ms:literal) => {
        tokio::spawn($crate::_set_interval(|| $cb, $ms));
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn test_set_timeout_macro_all_argument_variants_builds() {
        // macro takes expression
        set_timeout!(println!("hello1"), 4);
        // macro takes block
        set_timeout!({ println!("hello2") }, 3);
        // macro takes direct closure expressions
        set_timeout!(|| println!("hello3"), 2);
        // macro takes direct move closure expressions
        set_timeout!(move || println!("hello4"), 2);
        // macro takes identifiers (which must point to closures)
        let closure = || println!("hello5");
        set_timeout!(closure, 1);
    }

    #[tokio::test]
    async fn test_set_interval_macro_all_argument_variants_builds() {
        // macro takes expression
        set_interval!(println!("hello1"), 4);
        // macro takes block
        set_interval!({ println!("hello2") }, 3);
        // macro takes direct closure expressions
        set_interval!(|| println!("hello3"), 2);
        // macro takes direct move closure expressions
        set_interval!(move || println!("hello4"), 2);
        // macro takes identifiers (which must point to closures)
        let closure = || println!("hello5");
        set_interval!(closure, 1);
    }

    /// Test can't been automated because the test is correct
    /// if stdout is correct. Its just a visual test which can
    /// be executed manually.
    /// Output should be
    /// ```text
    /// hello1
    /// hello3
    /// hello5
    /// hello2
    /// hello4
    /// ```
    #[tokio::test]
    async fn test_set_timeout_visual() {
        println!("hello1");
        set_timeout!(println!("hello2"), 0);
        println!("hello3");
        set_timeout!(println!("hello4"), 0);
        println!("hello5");
        // give the timeout enough execution time
        tokio::time::sleep(Duration::from_millis(1)).await;
    }

    /// Test can't been automated because the test is correct
    /// if stdout is correct. Its just a visual test which can
    /// be executed manually.
    #[tokio::test]
    async fn test_set_interval_visual() {
        set_interval!(println!("should be printed 3 times"), 2);
        // give the timeout enough execution time
        tokio::time::sleep(Duration::from_millis(7)).await;
    }

    #[tokio::test]
    async fn test_set_timeout() {
        let counter = Arc::new(AtomicU64::new(0));
        {
            let counter = counter.clone();
            // the block is required to change the return type to "()"
            set_timeout!(
                move || {
                    counter.fetch_add(1, Ordering::SeqCst);
                },
                50
            );
        }
        {
            let counter = counter.clone();
            // the block is required to change the return type to "()"
            set_timeout!(
                move || {
                    counter.fetch_add(1, Ordering::SeqCst);
                },
                50
            );
        }

        // give the tokio runtime enough execution time
        tokio::time::sleep(Duration::from_millis(110)).await;
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_set_interval() {
        let counter = Arc::new(AtomicU64::new(0));
        {
            let counter = counter.clone();
            // the block is required to change the return type to "()"
            set_interval!(
                move || {
                    counter.fetch_add(1, Ordering::SeqCst);
                },
                50
            );
        }

        // give the tokio runtime enough execution time to execute the interval 3 times
        tokio::time::sleep(Duration::from_millis(180)).await;
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
}
