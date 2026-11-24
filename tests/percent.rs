use humany::{percent, Locale, RoundingMode};

#[test]
fn spec_table() {
    assert_eq!(format!("{}", percent(0.5)), "50%");
    assert_eq!(format!("{}", percent(0.125)), "12.5%");
    assert_eq!(format!("{}", percent(1.0)), "100%");
    assert_eq!(format!("{}", percent(0.85623).decimals(2)), "85.62%");
}

#[test]
fn negative() {
    assert_eq!(format!("{}", percent(-0.125)), "-12.5%");
}

#[test]
fn rounds_decimal_values_without_losing_the_sign() {
    assert_eq!(percent(-0.005).to_string(), "-0.5%");
    assert_eq!(percent(-0.00001).to_string(), "0%");
    assert_eq!(
        percent(0.29)
            .decimals(0)
            .rounding(RoundingMode::Floor)
            .to_string(),
        "29%"
    );
    assert_eq!(
        percent(0.01005)
            .rounding(RoundingMode::HalfAwayFromZero)
            .to_string(),
        "1.01%"
    );
    assert_eq!(
        percent(f64::from_bits(1))
            .decimals(18)
            .rounding(RoundingMode::Ceil)
            .to_string(),
        "0.000000000000000001%"
    );
}

#[test]
fn precision_and_range() {
    assert_eq!(
        percent(0.5)
            .decimals(255)
            .trim_trailing_zeros(false)
            .to_string(),
        "50.000000000000000000%"
    );
    assert_eq!(percent(0.005).decimals(21).to_string(), "0.5%");
    assert_eq!(percent(1e18).to_string(), "100000000000000000000%");
    assert_eq!(
        percent(-92_233_720_368_547_760.0).to_string(),
        "-9223372036854776000%"
    );
    assert_eq!(
        percent(f64::MAX).to_string(),
        format!("17976931348623157{}%", "0".repeat(294))
    );
    assert_eq!(percent(f64::INFINITY).to_string(), "inf%");
    assert_eq!(percent(f64::NAN).to_string(), "NaN%");
}

#[test]
fn localized_and_custom_symbols() {
    assert_eq!(
        percent(0.125).locale(Locale::Fr).to_string(),
        "12,5\u{00a0}%"
    );
    assert_eq!(percent(0.125).locale(Locale::Ar).to_string(), "12٫5٪");
    assert_eq!(
        percent(0.125).locale(Locale::Ar).symbol("%").to_string(),
        "12٫5%"
    );
}
