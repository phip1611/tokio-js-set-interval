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
//! `setTimeout(callback, ms)` as in Javascript inside a `tokio` runtime (<https://tokio.rs/>).
//! The library provides the macros:
//! - `set_interval!(callback, ms)`,
//! - `set_interval_async!(async_callback, ms)`,
//! - `set_timeout!(callback, ms)`,
//! - and `set_timeout_async!(async_callback, ms)`.
//!
//! ## Restrictions
//! They behave similar to their Javascript counterparts, with a few exceptions:
//!
//!  * They only get executed if the `tokio` runtime lives long enough.
//!  * on order to compile, the callback must return the union type, i.e. `()` \
//!    => all actions must be done via side effects
//!  * ⚠ again, there is **NO GUARANTEE** that the tasks will get executed \
//!    (=> but useful/convenient for low priority background tasks and for the learning effect of course) ⚠
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

use lazy_static::lazy_static;
use std::collections::HashSet;
use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use tokio::time::{Duration, Interval};

/// **INTERNAL** Use macro [`set_timeout`] instead!
///
/// Creates a future that glues the tokio sleep function (timeout) together with
/// the provided callback.
pub async fn _set_timeout(f: impl Fn(), ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    f();
}

/// **INTERNAL** Use macro [`set_timeout`] instead!
///
/// Async version of [_set_timeout].
pub async fn _set_timeout_async<T, F: Future<Output = T>>(f: impl Fn() -> F, ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    f().await;
}

/// **INTERNAL** Used to manage intervals created by macro [`set_interval`]!
/// Helps to assign unique IDs to intervals and stop them.
pub struct IntervalManager {
    /// Monotonic incrementing counter from 0 to max value used for interval IDs.
    pub counter: AtomicU64,
    /// Contains only the IDs of dispatched and not cleared intervals.
    pub running_intervals: Mutex<HashSet<u64>>,
}

lazy_static! {
    // TODO remove; Mutex::new now const since Rust 1.62 or so
    /// **INTERNAL** Used to manage intervals created by macro [`set_interval`]!
    pub static ref INTERVAL_MANAGER: IntervalManager = IntervalManager {
        counter: AtomicU64::new(0),
        running_intervals: Mutex::new(HashSet::new())
    };
}

/// Common code for [_set_interval] and [_set_interval_async]. Adds the new interval to the
/// interval manager.
async fn _set_interval_common(ms: u64, id: u64) -> Interval {
    // dedicated scope -> early drop lock
    {
        let mut map = INTERVAL_MANAGER.running_intervals.lock().unwrap();
        map.insert(id);
    }

    let mut int = tokio::time::interval(Duration::from_millis(ms));

    // the first tick in tokios interval returns immediately. Because we want behaviour
    // similar to Javascript in this library, we work around this.
    int.tick().await;

    int
}

/// Creates a future that glues the tokio interval function together with
/// the provided callback. Helper function for [`_set_interval_spawn`].
async fn _set_interval(f: impl Fn() + Send + 'static, ms: u64, id: u64) {
    let mut int = _set_interval_common(ms, id).await;

    // this looks like it runs for ever, but this is not how tokio works
    // at least with tokio 1.6.0 and Rust 1.52 this stops executing
    // when the tokio runtime gets dropped.
    loop {
        int.tick().await;

        // - breaks the loop if the interval was cleared.
        // - dedicated block to drop lock early
        {
            let map = INTERVAL_MANAGER.running_intervals.lock().unwrap();
            if !map.contains(&id) {
                break;
            }
        }

        f();
    }
}

/// Async version of [_set_interval].
async fn _set_interval_async<T, Func, Fut>(f: Func, ms: u64, id: u64)
where
    Func: (Fn() -> Fut) + Send + 'static + Sync,
    Fut: Future<Output = T> + Send + Sync,
{
    let mut int = _set_interval_common(ms, id).await;

    // this looks like it runs for ever, but this is not how tokio works
    // at least with tokio 1.6.0 and Rust 1.52 this stops executing
    // when the tokio runtime gets dropped.
    loop {
        int.tick().await;

        // - breaks the loop if the interval was cleared.
        // - dedicated block to drop lock early
        {
            let map = INTERVAL_MANAGER.running_intervals.lock().unwrap();
            if !map.contains(&id) {
                break;
            }
        }

        f().await;
    }
}

/// **INTERNAL** Use macro [`set_interval`] instead!
///
/// Acquires a new ID, creates an interval future and returns the ID. The future gets
/// dispatched as tokio task.
pub fn _set_interval_spawn(f: impl Fn() + Send + 'static, ms: u64) -> u64 {
    // assign next ID to interval
    let id = INTERVAL_MANAGER.counter.fetch_add(1, Ordering::SeqCst);
    tokio::spawn(_set_interval(f, ms, id));
    id
}

/// **INTERNAL** Use macro [`set_interval_async`] instead!
///
/// Async version of [_set_interval_spawn].
pub fn _set_interval_spawn_async<T: 'static, Func, Fut>(f: Func, ms: u64) -> u64
where
    Func: (Fn() -> Fut) + Send + 'static + Sync,
    Fut: Future<Output = T> + Send + 'static + Sync,
{
    // assign next ID to interval
    let id = INTERVAL_MANAGER.counter.fetch_add(1, Ordering::SeqCst);
    tokio::spawn(_set_interval_async(f, ms, id));
    id
}

/// Clears an interval that was created via [`set_interval!`]. You have to pass
/// the unique numeric ID as parameter. Throws no error, if the ID is unknown.
pub fn clear_interval(id: u64) {
    let mut set = INTERVAL_MANAGER.running_intervals.lock().unwrap();
    if set.contains(&id) {
        set.remove(&id);
    }
}

/// Creates a timeout that behaves similar to `setTimeout(callback, ms)` in Javascript
/// for the `tokio` runtime. Unlike in Javascript, it will only be executed, if after the
/// specified time passed, the `tokio` runtime still lives, i.e. didn't get dropped.
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
        $crate::set_timeout!(|| $cb, $ms);
    };
    // match for block
    ($cb:block, $ms:literal) => {
        $crate::set_timeout!(|| $cb, $ms);
    };
}

/// Async version of [set_timeout].
///
/// # Example
/// ```rust
/// use tokio::time::Duration;
/// use tokio_js_set_interval::set_timeout_async;
///
/// async fn async_foo() {
///     println!("hello1");
/// }
///
/// #[tokio::main]
/// async fn main() {
///     set_timeout_async!(async_foo, 0);
///     println!("hello2");
///     // prevent that tokios runtime gets dropped too early
///     // order of output should be
///     //  "hello2"
///     //  "hello1"
///     tokio::time::sleep(Duration::from_millis(1)).await;
/// }
/// ```
#[macro_export]
macro_rules! set_timeout_async {
    // match for identifier, i.e. a closure, that is behind a variable
    ($cb:ident, $ms:literal) => {
        tokio::spawn($crate::_set_timeout_async($cb, $ms));
    }; /* Async closures not supported currently:
          https://github.com/rust-lang/rust/issues/62290
       // match for direct closure expression
       (async || $cb:expr, $ms:literal) => {
           tokio::spawn($crate::_set_timeout_async(async || $cb, $ms));
       };
       // match for direct move closure expression
       (move async || $cb:expr, $ms:literal) => {
           tokio::spawn($crate::_set_timeout_async(move async || $cb, $ms));
       };


       /* There are no async expressions, I think
       // match for expr, like `set_timeout!(println!())`
       ($cb:expr, $ms:literal) => {
           $crate::set_timeout_async!(|| $cb, $ms);
       };*/

       // match for async block
       (async $cb:block, $ms:literal) => {
           $crate::set_timeout_async!(async || $cb, $ms);
       };
       */
}

/// Creates a timeout that behaves similar to `setInterval(callback, ms)` in Javascript
/// for the `tokio` runtime. Unlike in Javascript, it will only be executed, if after the
/// specified time passed, the `tokio` runtime still lives, i.e. didn't got dropped.
///
/// As in Javascript, an interval may only have side effects and no return type.
/// You don't get a handle to manually wait for it, you must ensure, that the tokio
/// runtime lives long enough.
///
/// The macro returns an numeric ID. Similar to Javascript, you can use thie ID to
/// clear/stop intervals with [`clear_interval`].
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
///     let interval = set_interval!(println!("hello1"), 50);
///     // If you want to clear the interval later: save the ID
///     // let id = set_interval!(println!("hello1"), 50);
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
        // not so nice, need to wrap the identifier in another closure
        $crate::set_interval!(move || $cb(), $ms)
    };
    // match for direct closure expression
    (|| $cb:expr, $ms:literal) => {
        $crate::_set_interval_spawn(|| $cb, $ms)
    };
    // match for direct move closure expression
    (move || $cb:expr, $ms:literal) => {
        $crate::_set_interval_spawn(move || $cb, $ms)
    };
    // match for expr, like `set_interval!(println!())`
    ($cb:expr, $ms:literal) => {
        $crate::set_interval!(|| $cb, $ms)
    };
    // match for block
    ($cb:block, $ms:literal) => {
        $crate::set_interval!(|| $cb, $ms)
    };
}

/// Async version of [set_interval].
///
/// # Example
/// ```rust
/// use tokio::time::Duration;
/// use tokio_js_set_interval::set_interval_async;
///
/// #[tokio::main]
/// async fn main() {
///     async fn async_foo() {
///         println!("hello1")
///     }
///
///     set_interval_async!(async_foo, 50);
///     // If you want to clear the interval later: save the ID
///     // let id = set_interval_async!(async_foo, 50);
///     println!("hello2");
///     // prevent that tokios runtime gets dropped too early
///     // "hello1" should get printed 2 times (50*2 == 100 < 120)
///     tokio::time::sleep(Duration::from_millis(120)).await;
/// }
/// ```
#[macro_export]
macro_rules! set_interval_async {
    // match for identifier, i.e. a closure, that is behind a variable
    ($cb:ident, $ms:literal) => {
        $crate::_set_interval_spawn_async($cb, $ms)
    }; /* Async closures not supported currently:
          https://github.com/rust-lang/rust/issues/62290
       // match for direct closure expression
       (|| $cb:expr, $ms:literal) => {
           $crate::_set_interval_spawn(|| $cb, $ms)
       };
       // match for direct move closure expression
       (move || $cb:expr, $ms:literal) => {
           $crate::_set_interval_spawn(move || $cb, $ms)
       };
       // match for block
       ($cb:block, $ms:literal) => {
           $crate::set_interval!(|| $cb, $ms)
       };*/
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicU64;
    use std::sync::Arc;

    use super::*;

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

        /* Async closures not supported currently:
           https://github.com/rust-lang/rust/issues/62290
        // macro takes block.
        set_timeout_async!(async { println!("hello2") }, 3);
        // macro takes direct closure expressions
        set_timeout_async!(async || println!("hello3"), 2);
        // macro takes direct move closure expressions
        set_timeout_async!(move async || println!("hello4"), 2);
        */

        // macro takes identifiers (which must point to closures)
        async fn async_foo() {
            println!("hello5");
        }
        set_timeout_async!(async_foo, 1);
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

        // macro takes identifiers (which must point to closures)
        async fn async_foo() {
            println!("hello5");
        }
        set_interval_async!(async_foo, 1);
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

        // give enough execution time, before tokio gets dropped
        // two tokio sleeps: timeouts will usually get scheduled in between
        tokio::time::sleep(Duration::from_millis(1)).await;
        tokio::time::sleep(Duration::from_millis(1)).await;
    }

    /// Test can't been automated because the test is correct
    /// if stdout is correct. Its just a visual test which can
    /// be executed manually.
    #[tokio::test]
    async fn test_set_interval_visual() {
        let id = set_interval!(println!("should be printed 3 times"), 50);
        // give the timeout enough execution time
        tokio::time::sleep(Duration::from_millis(151)).await;
        tokio::time::sleep(Duration::from_millis(0)).await;
        println!("interval id is: {}", id);
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
    async fn test_set_timeout_async() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        async fn async_foo() {
            COUNTER.fetch_add(1, Ordering::SeqCst);
        }
        {
            set_timeout_async!(async_foo, 50);
        }
        {
            set_timeout_async!(async_foo, 50);
        }

        // give the tokio runtime enough execution time
        tokio::time::sleep(Duration::from_millis(110)).await;
        assert_eq!(COUNTER.load(Ordering::SeqCst), 2);
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

    #[tokio::test]
    async fn test_set_interval_async() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        async fn async_foo() {
            COUNTER.fetch_add(1, Ordering::SeqCst);
        }
        {
            // the block is required to change the return type to "()"
            set_interval_async!(async_foo, 50);
        }

        // give the tokio runtime enough execution time to execute the interval 3 times
        tokio::time::sleep(Duration::from_millis(180)).await;
        assert_eq!(COUNTER.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_clear_interval() {
        let counter = Arc::new(AtomicU64::new(0));
        let interval_id;
        {
            let counter = counter.clone();
            // the block is required to change the return type to "()"
            interval_id = set_interval!(
                move || {
                    counter.fetch_add(1, Ordering::SeqCst);
                },
                50
            );
        }

        // give the tokio runtime enough execution time to execute the interval 3 times
        tokio::time::sleep(Duration::from_millis(180)).await;
        assert_eq!(counter.load(Ordering::SeqCst), 3);
        clear_interval(interval_id);
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
}
