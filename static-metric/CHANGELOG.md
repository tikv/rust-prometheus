# Changelog for prometheus-static-metric

## 0.5.0 [unreleased]

- Bug fix: Allow not specifying visibility token (e.g. `pub(crate)`) for metric definition.

## 0.4.0

- Misc: Update dependencies (https://github.com/tikv/rust-prometheus/pull/289, https://github.com/tikv/rust-prometheus/pull/311)
- Add: Local metric trait (https://github.com/tikv/rust-prometheus/pull/297)
- Add: Auto-flushable thread local metrics (https://github.com/tikv/rust-prometheus/pull/304)
- Add: Derive `Eq` and `Hash` for label enums (https://github.com/tikv/rust-prometheus/pull/317)

## 0.3.0

- Misc: Update the dependent Syn version (https://github.com/tikv/rust-prometheus/pull/268)
- Misc: Requires rust-prometheus v0.7+ (https://github.com/tikv/rust-prometheus/pull/252)

## 0.2.0

- Add: Local static metric (https://github.com/tikv/rust-prometheus/pull/245)

## 0.1.4

- Add: Static metric enums support `get_str()` (https://github.com/tikv/rust-prometheus/pull/183)

## 0.1.3

- Change: Static metrics are now type safe (https://github.com/tikv/rust-prometheus/pull/182)

## 0.1.2

- Misc: Update the dependent Protobuf version (https://github.com/tikv/rust-prometheus/pull/181)

## 0.1.1

- Add: `register_static_xxx!` macros (https://github.com/tikv/rust-prometheus/pull/180)
