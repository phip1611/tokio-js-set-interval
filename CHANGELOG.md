# 1.3.0 (2023-06-25)
- MSRV is `1.66`
- Minimum Tokio version is still `1.0`. Note that Rust `>= 1.73` will deny tokio `<1.18`.
- Remove dependency to `lazy_static`

# 1.2.1 (2023-06-25)
- All macros now accept an identifier as parameter for the period. For example:
  ```rust
  let period = 5000;
  set_interval!(println!("hey!"), period);
  ```

# 1.2.0 (2022-09-12)
- Added the `set_timeout_async!` macro. It accepts futures either by expression or by identifier.
- Added the `set_interval_async!` macro. It accepts a function that produces futures by identifier
  or by expression.
- examples for the new macros can be found in `examples/macro-possible-arguments.rs`
- small fix in `set_interval!`
- FYI: MSRV is `1.49.0` (with `tokio @ 1.21.0`). Minimum supported tokio version is still `1.0.x`.

# 1.1.1 (2022-05-02)
- small internal improvements

# 1.1.0 (2021-06-14)
- began with changelog
- added `clear_interval`-function to clear intervals, similar to javascript
- `set_interval` now returns an integer (ID) -> used as value for clear_interval

# 1.0.1, 1.0.2, 1.0.3 (2021-05-17)
- README updates
- text fixes

# 1.0.0 (2021-05-17)
- initial release
