# 1.2.0 (2022-09-04)
- Added the `set_timeout_async!` macro. It accepts futures either by expression or by identifier.
- Added the `set_interval_async!` macro. It accepts a function that produces futures by identifier
  or by expression.
- small fix in `set_interval!`

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
