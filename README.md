# `tokio-js-set-interval`

The crate `tokio-js-set-interval` allows you to use `setInterval(callback, ms)` and
`setTimeout(callback, ms)` as in Javascript inside a **`tokio` runtime (https://tokio.rs/)**.
The library provides the macros:
- `set_interval!(callback, ms)`,
- `set_interval_async!(future, ms)`,
- `set_timeout!(callback, ms)`,
- and `set_timeout_async!(async_callback, ms)`.

## How to use
**Cargo.toml**
```toml
[dependencies]
# don't forget that you also need "tokio" for the tokio runtime!
tokio-js-set-interval = "<latest-version>"
```

**code.rs**
```rust
use std::time::Duration;
use tokio_js_set_interval::{set_interval, set_timeout, clear_interval};

#[tokio::main]
async fn main() {
    set_timeout!(println!("hello from timeout"), 25);
    set_interval!(println!("hello from interval"), 10);
    // you can clear intervals if you want
    let id = set_interval!(println!("hello from interval"), 10);
    clear_interval(id);

    // give enough time before tokios runtime exits
    tokio::time::sleep(Duration::from_millis(40)).await;
}
```

## Restrictions
They behave similar to their Javascript counterparts, with a few exceptions:

 * They only get executed if the `tokio` runtime lives long enough.
 * on order to compile, the callback must return the union type, i.e. `()` \
   => all actions must be done via side effects
 * again, there is **NO GUARANTEE** that the tasks will get executed \
   (=> however useful and convenient for low priority background tasks and for the learning effect of course)

## Trivia
I'm not an expert in `tokio` (or async/await/futures in Rust in general) and I don't
know if this follows best practises. But it helped me to understand how `tokio` works.
I hope it may be helpful to some of you too.

The functionality behind is rather simple. However, it took me some time to figure out what kind of
input the macros should accept and how the generic arguments of the functions behind the macros
need to be structured. Especially the `*_async!()` versions of the macros were quite complicated
during the development.

## Compatibility & MSRV
Version 1.2.0 is developed with `tokio@1.21.0` and `rust@1.63.0`. The minimum supported `tokio`
version is `1.0.x` and the MSRV depends on the `tokio` version. For the latest tokio version,
the `MSRV` is `1.49.0`.
