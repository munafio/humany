# Contributing to Humany

Thanks for helping make Humany better. This project is small on purpose — keep changes focused and well tested.

## Before you start

1. Open an [issue](https://github.com/munafio/humany/issues) for bugs or larger features (typos and tiny fixes can skip this).
2. Read [SECURITY.md](SECURITY.md) before reporting vulnerabilities — do **not** open public issues for security problems.

## Development setup

```bash
git clone https://github.com/munafio/humany.git
cd humany
cargo test
cargo test --no-default-features
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

**MSRV:** Rust **1.70** (see `rust-version` in `Cargo.toml`).

## What we welcome

- Bug fixes with a test that failed before the fix
- New locales — add `src/locale/langs/xx.rs` and register in `locale/mod.rs` + `tests/locale.rs`
- Docs and README improvements
- Performance improvements that do not hurt readability

## Pull request checklist

- [ ] `cargo test` and `cargo test --no-default-features` pass
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo fmt --all` applied
- [ ] Public API changes have `///` docs (`#![deny(missing_docs)]` on the crate)
- [ ] New locale strings have golden tests in `tests/locale.rs`
- [ ] CHANGELOG.md updated under **Unreleased** (if user-visible)

## Code style

- Match existing modules: lazy `Display`, no unnecessary `alloc`, `Copy` builders.
- Locale copy lives in `src/locale/langs/` — shared logic stays in `src/locale/`.
- Prefer small, reviewable PRs over large rewrites.

## Commits

Use clear messages in imperative mood, e.g. `fix bytes tier for values under 1 KiB`, `add Locale::It`.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).