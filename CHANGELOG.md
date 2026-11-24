# Changelog

## 0.1.1 — Unreleased

- Preserve signed rounding and large numeric values. Clamp formatting precision to 18 decimal places and handle empty byte-unit tables.
- Preserve pre-1970 timestamps and wide timestamp differences. Correct relative-time wording and localize decimal separators.
- Include subsecond components in long durations. Deprecate the unused duration precision and rounding methods.
- Remove the mandatory `libm` dependency and Chrono's unused `clock` feature.

## 0.1.0

- Initial release: number, bytes, duration, relative, percent formatters.
- `no_std` core with optional `std`, `chrono`, and `time` features.
- `Humany` context and nine `Locale`s (`En`, `Ar`, `Fr`, `Es`, `De`, `Zh`, `Ja`, `Pt`, `Ru`).
- MIT license.
