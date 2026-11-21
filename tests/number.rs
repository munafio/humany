use humany::{number, Humany, Locale};

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
    assert_eq!(ar, "1.5M");
}

#[test]
fn humany_instance() {
    assert_eq!(format!("{}", Humany::DEFAULT.number(1_000)), "1K");
}
