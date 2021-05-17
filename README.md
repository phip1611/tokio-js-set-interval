# `tokio-js-set-interval`

The crate `tokio-js-set-interval` allows you to use `setInterval(callback, ms)` and
`setTimeout(callback, ms)` as in Javascript inside a **`tokio` runtime (https://tokio.rs/)**.
For this, it offers the macros `set_interval!(callback, ms)` and `set_timeout!(callback, ms)`.

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
use tokio_js_set_interval::{set_interval, set_timeout};

#[tokio::main]
async fn main() {
    set_timeout!(println!("hello from timeout"), 25);
    set_interval!(println!("hello from interval"), 10);

    // give enough time before tokios runtime exits
    tokio::time::sleep(Duration::from_millis(40)).await;
}
```

## Restrictions
They behave similar to their Javascript counterparts, with a few exceptions:

 * They only get executed if the `tokio` runtime lives long enough.
 * on order to compile, the callback must return the union type, i.e. `()`
 * => all actions must be done via side effects
 * again, there is **NO GUARANTEE** that the tasks will get executed \
   (--> but useful/convenient for low priority background tasks and for the learning effect of course)

## Trivia
⚠ I'm not an expert in `tokio` (or async/await/futures in Rust in general) and I don't
  know if this follows best practises. But it helped me to understand how `tokio` works.
  I hope it may be helpful to some of you too. ⚠

The functionality itself is really simple. The biggest part are the convenient macros.
I over-engineered them a little to learn more about macros.

## Compatibility
Version 1.0.0 is developed with:
 * `tokio` @ 1.6.0 (but should also work with 1.0.0)
 * `rustc` @ 1.52.1 (but should also work with 1.45.2)


## Changelog
- Releases 1.0.1, 1.0.2, 1.0.3 (2021-05-17)
  - README updates
  - text fixes
- Release 1.0.0 (2021-05-17)
    - initial release