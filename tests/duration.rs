use core::time::Duration as CoreDuration;
use humany::{duration, DurationUnit, Locale};

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

#[test]
fn long_output_keeps_subseconds_and_respects_limits() {
    assert_eq!(
        duration(CoreDuration::from_millis(500)).long().to_string(),
        "500 milliseconds"
    );
    let value = CoreDuration::new(1, 1_002_003);
    assert_eq!(
        duration(value).long().to_string(),
        "1 second 1 millisecond 2 microseconds 3 nanoseconds"
    );
    assert_eq!(
        duration(value)
            .long()
            .min_unit(DurationUnit::Microsecond)
            .max_units(2)
            .separator('/')
            .to_string(),
        "1 second/1 millisecond"
    );
    assert_eq!(
        duration(value)
            .min_unit(DurationUnit::Millisecond)
            .to_string(),
        "1s 1ms"
    );
    assert_eq!(
        duration(59_u64).min_unit(DurationUnit::Minute).to_string(),
        "0m"
    );
    assert_eq!(duration(59_u64).max_units(0).to_string(), "0s");
    assert_eq!(duration(0_u64).locale(Locale::Ja).to_string(), "0秒");
    assert_eq!(
        duration(CoreDuration::from_millis(22))
            .long()
            .locale(Locale::Ru)
            .to_string(),
        "22 миллисекунды"
    );
}

#[test]
fn largest_duration_uses_whole_seconds_without_overflow() {
    let duration = duration(CoreDuration::new(u64::MAX, 999_999_999));
    assert_eq!(
        duration.to_string(),
        "213503982334601d 7h 15s 999ms 999µs 999ns"
    );
}
