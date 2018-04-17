# Changelog

## 0.4.1

- Add: `(Local)(Int)Counter::try_inc_by()` for users who want to handle counter decrease errors instead of panic (#165).

## 0.4.0

- Add: `IntCounter`, `IntCounterVec`, `IntGauge`, `IntGaugeVec`, `LocalIntCounter`, `LocalIntCounterVec` for better performance when metric values are all integers (#158).

- Change: When the given value is < 0, `(Local)Counter::inc_by()` no longer return errors, instead it will panic (#156).

- Improve performance (#161).
