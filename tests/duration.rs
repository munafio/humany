use core::time::Duration as CoreDuration;
use humany::duration;

#[test]
fn compact_spec() {
    assert_eq!(format!("{}", duration(5_u64)), "5s");
    assert_eq!(format!("{}", duration(65_u64)), "1m 5s");
    assert_eq!(format!("{}", duration(3_661_u64)), "1h 1m 1s");
    assert_eq!(format!("{}", duration(90_061_u64)), "1d 1h 1m 1s");
}

#[test]
fn max_units() {
    assert_eq!(format!("{}", duration(3665_u64).max_units(2)), "1h 1m");
}

#[test]
fn subsecond() {
    let d = CoreDuration::new(1, 500_000_000);
    assert_eq!(format!("{}", duration(d)), "1s 500ms");
}
