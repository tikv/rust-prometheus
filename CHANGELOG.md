# Changelog

## 0.7.0

- Provide immutable interface for local counters.

## 0.6.1

- Fix compile error when ProtoBuf feature is not enabled (#240).

## 0.6.0

- Add: Expose the default registry (#231).

- Add: Support common namespace prefix and common labels (#233).

## 0.5.0

- Change: Added TLS and BasicAuthentication support to `push` client.

## 0.4.2

- Change: Update to use protobuf 2.0.

## 0.4.1

- Change: `(Local)(Int)Counter.inc_by` only panics in debug build if the given value is < 0 (#168).

## 0.4.0

- Add: Provides `IntCounter`, `IntCounterVec`, `IntGauge`, `IntGaugeVec`, `LocalIntCounter`, `LocalIntCounterVec` for better performance when metric values are all integers (#158).

- Change: When the given value is < 0, `(Local)Counter.inc_by` no longer return errors, instead it will panic (#156).

- Improve performance (#161).
