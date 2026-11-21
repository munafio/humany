

# Humany

### Human-readable formatting for Rust

**Turn raw numbers, bytes, and timestamps into strings people actually read — lazy `Display`, `no_std` first, optional `std`.**

  


![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?style=flat-square&logo=rust)
![no_std](https://img.shields.io/badge/no__std-ready-546e7a?style=flat-square)
![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![MSRV](https://img.shields.io/badge/MSRV-1.70-informational?style=flat-square)
![GitHub](https://img.shields.io/badge/GitHub-munafio%2Fhumany-24292f?style=flat-square&logo=github)

`formatting` · `duration` · `bytes` · `i18n` · `display` · `embedded`

  


[Quick start](#quick-start) ·
[Formatters](#formatters) ·
[Locales](#locales) ·
[Features](#cargo-features) ·
[Contributing](CONTRIBUTING.md)



---

## What is Humany?

Computers like `1250000` and `1073741824`. Humans like `**1.25M**` and `**1 GiB**`.

**Humany** is a small Rust crate that:

- Formats values **only when you print them** (`Display` — no extra `String` unless you ask).
- Works in `**no_std**` projects (with a tiny `libm` helper for floats).
- Lets you set language **once** with [`Humany`](src/context.rs) presets, or **per value** with `.locale(...)`.

If you have ever copy-pasted “format bytes” helpers into three projects — this is the one dependency instead.

---

## Quick start

Add to `Cargo.toml`:

```toml
[dependencies]
humany = "0.1"
```

**Default** build includes `std` (for `SystemTime` relative times). For embedded:

```toml
humany = { version = "0.1", default-features = false }
```

Minimal example:

```rust
use humany::{bytes, duration, number, percent, relative_secs, Humany};

fn main() {
    println!("{}", number(1_250_000));           // 1.25M
    println!("{}", bytes(1_024));                // 1 KiB
    println!("{}", duration(3_665_u64));         // 1h 1m 5s
    println!("{}", percent(0.85623));             // 85.62% (default precision)
    println!("{}", relative_secs(120));          // 2 minutes ago

    // One locale for the whole block
    let ar = Humany::AR;
    println!("{}", ar.relative_secs(120));       // منذ دقيقتين
}
```

Run the included examples:

```bash
cargo run --example basic          # defaults only
cargo run --example customization  # defaults, then chained options
```

---

## Formatters

Each formatter is a small **builder**: pass a value, `println!` it, and only chain methods when you need something other than the defaults (decimals, locale, units, style, …). All builders implement `Display`.

**Pattern:** `println!("{}", thing(value));` — defaults · `println!("{}", thing(value).option());` — customize.

### Numbers — `number()`

| Input | Output |
|------:|--------|
| 999 | `999` |
| 1_000 | `1K` |
| 1_250_000 | `1.25M` |
| 1_500_000_000 | `1.5B` |
| -1_500_000 | `-1.5M` |

Defaults: **2** fractional digits when scaling, trailing zeros trimmed (`1.5M` not `1.50M`).

```rust
use humany::number;

// Simple
println!("{}", number(1_250_000)); // 1.25M

// Customize when you need fixed width or more digits
println!("{}", number(1_250_000).decimals(3).trim_trailing_zeros(false)); // 1.250M
```

### Bytes — `bytes()`

Binary (IEC) by default; decimal (SI) with `.decimal()`.

| Input | Default | `.decimal()` |
|------:|---------|--------------|
| 512 | `512 B` | `512 B` |
| 1_024 | `1 KiB` | `1 kB` |
| 1_500_000 | `1.43 MiB` | `1.5 MB` |

```rust
use humany::bytes;

// Simple — binary KiB / MiB / GiB
println!("{}", bytes(1_024));        // 1 KiB
println!("{}", bytes(1_500_000));   // 1.43 MiB

// Customize — SI decimals (kB, MB) or precision
println!("{}", bytes(1_500_000).decimal()); // 1.5 MB
```

### Duration — `duration()`

Accepts **seconds** (`u64`) or `std::time::Duration` (includes sub-second parts).

| Seconds | Compact (default) |
|--------:|-------------------|
| 5 | `5s` |
| 65 | `1m 5s` |
| 3_665 | `1h 1m 5s` |

```rust
use humany::duration;

// Simple
println!("{}", duration(3_665_u64)); // 1h 1m 5s

// Customize — fewer components or long labels
println!("{}", duration(3_665_u64).max_units(2)); // 1h 1m
println!("{}", duration(3_665_u64).long());       // 1 hour 1 minute 5 seconds
```

### Relative time — `relative_secs()` / `relative()`

| Delta | English (default) |
|------:|-------------------|
| 120 | `2 minutes ago` |
| -172_800 | `in 2 days` |

```rust
use humany::{relative_secs, Locale};

// Simple
println!("{}", relative_secs(120)); // 2 minutes ago

// Customize — locale or compact style
println!("{}", relative_secs(120).locale(Locale::Ar)); // منذ دقيقتين
println!("{}", relative_secs(120).compact());          // 2m ago
```

With the `std` feature and a wall clock:

```rust
use humany::relative;
use std::time::{Duration, SystemTime};

let t = SystemTime::now() - Duration::from_secs(120);

// Simple — compare to now when printed
println!("{}", relative(t));

// Customize — fixed reference for tests or snapshots
println!("{}", relative(t).reference(SystemTime::now()));
```

### Percent — `percent()`

| Ratio | Default output |
|------:|----------------|
| 0.5 | `50%` |
| 0.85623 | `85.62%` |
| 0.125 | `12.5%` |

Defaults: multiply by 100, **2** decimals, trim trailing zeros.

```rust
use humany::percent;

// Simple
println!("{}", percent(0.125));    // 12.5%
println!("{}", percent(0.85623));  // 85.62%

// Customize — more digits or symbols
println!("{}", percent(0.85623).decimals(4)); // 85.6230%
```

---

## Locales

Built-in languages:

| Code | `Locale` | `Humany` preset | Example (2 min ago) |
|------|----------|-----------------|---------------------|
| English | `Locale::En` | `Humany::DEFAULT` | `2 minutes ago` |
| Arabic | `Locale::Ar` | `Humany::AR` | `منذ دقيقتين` |
| French | `Locale::Fr` | `Humany::FR` | `il y a 2 minutes` |
| Spanish | `Locale::Es` | `Humany::ES` | `hace 2 minutos` |
| German | `Locale::De` | `Humany::DE` | `vor 2 Minuten` |
| Chinese (Simplified) | `Locale::Zh` | `Humany::ZH` | `2分钟前` |
| Japanese | `Locale::Ja` | `Humany::JA` | `2分前` |
| Portuguese | `Locale::Pt` | `Humany::PT` | `há 2 minutos` |
| Russian | `Locale::Ru` | `Humany::RU` | `2 минут назад` |

**Two ways to set language:**

1. **Instance** — create once, reuse: `let h = Humany::FR;`
2. **Inline** — one-off: `number(n).locale(Locale::Ja)`

Inline `.locale()` **overrides** the `Humany` instance if both are used.

Translation files live under [`src/locale/langs/`](src/locale/langs/). See [CONTRIBUTING.md](CONTRIBUTING.md) to add a language.

---

## `Humany` context

[`Humany`](src/context.rs) is a `Copy` config bundle: locale + default decimal/rounding options.

```rust
use humany::{Humany, Locale, FormatOptions};

let app = Humany::new()
    .locale(Locale::De)
    .decimals(1);

println!("{}", app.bytes(1_073_741_824));
println!("{}", app.relative_secs(3600));
```

Free functions (`number()`, `bytes()`, …) use `**Humany::DEFAULT**` (English).

---

## Cargo features

| Feature | Default? | Purpose |
|---------|----------|---------|
| `std` | yes | `relative(SystemTime)`, examples with `println!` |
| `chrono` | no | `relative_chrono(DateTime<Utc>)` |
| `time` | no | `relative_time(OffsetDateTime)` |

```toml
# Embedded / no_std
humany = { version = "0.1", default-features = false }

# With chrono
humany = { version = "0.1", features = ["chrono"] }
```

---

## `no_std`

The crate uses `#![no_std]`. With `default-features = false`:

- Core formatters work (`number`, `bytes`, `duration`, `percent`, `relative_secs`).
- `**libm**` is used for floating-point formatting (no `std` math).
- Enable `std` when you need wall-clock `SystemTime`.

```bash
cargo test --no-default-features
```

---

## Development

```bash
cargo test
cargo test --no-default-features
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cargo doc --open
```

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a PR.

---

## Philosophy

- **Raw values are for computers. Readable values are for humans.**
- Lazy formatting, predictable defaults, optional customization.
- No mandatory `alloc`; locales use static strings (`no_std`-friendly).

---

## License

[MIT License](LICENSE) © Humany contributors.

---



**[Report a bug](.github/ISSUE_TEMPLATE/bug_report.yml)** · **[Request a feature](.github/ISSUE_TEMPLATE/feature_request.yml)** · **[Security](SECURITY.md)**