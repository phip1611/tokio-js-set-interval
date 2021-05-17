# `tokio-js-set-interval`

The crate `tokio-js-set-interval` allows you to use `setInterval(callback, ms)` and
`setTimeout(callback, ms)` as in Javascript inside a `tokio` runtime (https://tokio.rs/).
For this, it offers the macros `set_interval!(callback, ms)` and `set_timeout!(callback, ms)`.

## Restrictions
They behave similar to their Javascript counterparts, with a few exceptions:

 * They only get executed if the `tokio` runtime lives long enough.
 * on order to compile, the callback must return the union type, i.e. `()`
 * => all actions must be done via side effects

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
