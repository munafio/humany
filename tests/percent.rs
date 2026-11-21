use humany::percent;

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
