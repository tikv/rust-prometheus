# Changelog

## 0.4.0

- Add: Provides `IntCounter`, `IntCounterVec`, `IntGauge`, `IntGaugeVec`, `LocalIntCounter`, `LocalIntCounterVec` for better performance when metric values are all integers (#158).

- Change: When the given value is < 0, `Counter.inc_by` and `LocalCounter.inc_by` no longer return errors, instead it will panic (#156).

- Improve performance (#161).
