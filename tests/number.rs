use humany::{number, Humany, Locale, RoundingMode};

#[test]
fn spec_table() {
    assert_eq!(format!("{}", number(999)), "999");
    assert_eq!(format!("{}", number(1_000)), "1K");
    assert_eq!(format!("{}", number(1_250_000)), "1.25M");
    assert_eq!(format!("{}", number(1_000_000)), "1M");
    assert_eq!(format!("{}", number(1_000_000_000)), "1B");
    assert_eq!(format!("{}", number(1_500_000_000)), "1.5B");
    assert_eq!(format!("{}", number(-1_500_000)), "-1.5M");
}

#[test]
fn decimals() {
    assert_eq!(format!("{}", number(1_250_000).decimals(2)), "1.25M");
}

#[test]
fn locale_inline() {
    let en = format!("{}", number(1_500_000));
    let ar = format!("{}", number(1_500_000).locale(Locale::Ar));
    assert_eq!(en, "1.5M");
    assert_eq!(ar, "1٫5M");
}

#[test]
fn humany_instance() {
    assert_eq!(format!("{}", Humany::DEFAULT.number(1_000)), "1K");
}

#[test]
fn signed_rounding_matches_for_integer_and_float_inputs() {
    let cases = [
        (RoundingMode::HalfAwayFromZero, "-1.3K"),
        (RoundingMode::Floor, "-1.3K"),
        (RoundingMode::Ceil, "-1.2K"),
        (RoundingMode::Trunc, "-1.2K"),
    ];
    for (mode, expected) in cases {
        assert_eq!(
            number(-1250).decimals(1).rounding(mode).to_string(),
            expected
        );
        assert_eq!(
            number(-1250.0).decimals(1).rounding(mode).to_string(),
            expected
        );
    }
    assert_eq!(
        number(1100.0).rounding(RoundingMode::Ceil).to_string(),
        "1.1K"
    );
}

#[test]
fn integer_extremes_and_non_finite_floats() {
    assert_eq!(
        number(u64::MAX).decimals(18).to_string(),
        "18446744.073709551615T"
    );
    assert_eq!(
        number(i64::MIN).suffixes(&[]).to_string(),
        "-9223372036854775808"
    );
    assert_eq!(
        number(u64::MAX).suffixes(&[]).to_string(),
        "18446744073709551615"
    );
    assert_eq!(number(f64::NAN).to_string(), "NaN");
    assert_eq!(number(f64::INFINITY).to_string(), "inf");
    assert_eq!(number(f64::NEG_INFINITY).to_string(), "-inf");
}
