use humany::{relative_between, relative_secs, Humany, Locale};

#[test]
fn verbose_en() {
    assert_eq!(format!("{}", relative_secs(5)), "5 seconds ago");
    assert_eq!(format!("{}", relative_secs(120)), "2 minutes ago");
    assert_eq!(format!("{}", relative_secs(3600)), "1 hour ago");
    assert_eq!(format!("{}", relative_secs(86_400 * 3)), "3 days ago");
}

#[test]
fn future_en() {
    assert_eq!(format!("{}", relative_secs(-86_400 * 2)), "in 2 days");
}

#[test]
fn arabic_instance() {
    assert_eq!(format!("{}", Humany::AR.relative_secs(120)), "منذ دقيقتين");
}

#[test]
fn inline_locale_ar() {
    assert_eq!(
        format!("{}", relative_secs(120).locale(Locale::Ar)),
        "منذ دقيقتين"
    );
}

#[test]
fn timestamp_difference_uses_the_full_unsigned_range() {
    assert_eq!(
        relative_between(0, u64::MAX).to_string(),
        "213503982334601 days ago"
    );
    assert_eq!(
        relative_between(u64::MAX, 0).to_string(),
        "in 213503982334601 days"
    );
    assert_eq!(
        relative_between(1 << 63, 0).to_string(),
        "in 106751991167300 days"
    );
    assert_eq!(
        relative_secs(i64::MIN).to_string(),
        "in 106751991167300 days"
    );
}

#[cfg(feature = "std")]
#[test]
fn system_clock_reference_works_across_the_epoch() {
    use std::time::{Duration, UNIX_EPOCH};

    let earlier = UNIX_EPOCH - Duration::from_millis(1500);
    let later = UNIX_EPOCH + Duration::from_millis(500);
    assert_eq!(
        humany::relative(earlier).reference(later).to_string(),
        "2 seconds ago"
    );
    assert_eq!(
        humany::relative(later).reference(earlier).to_string(),
        "in 2 seconds"
    );
}

#[cfg(feature = "chrono")]
#[test]
fn chrono_retains_pre_epoch_timestamps() {
    use std::time::{Duration, UNIX_EPOCH};

    let timestamp = chrono::DateTime::from_timestamp(-2, 500_000_000).unwrap();
    let reference = UNIX_EPOCH + Duration::from_millis(500);
    assert_eq!(
        humany::relative_chrono(timestamp)
            .reference(reference)
            .to_string(),
        "2 seconds ago"
    );

    let earliest = chrono::DateTime::<chrono::Utc>::MIN_UTC;
    assert!(humany::relative_chrono(earliest)
        .reference(UNIX_EPOCH)
        .to_string()
        .ends_with(" days ago"));
}

#[cfg(feature = "time")]
#[test]
fn time_retains_pre_epoch_timestamps() {
    use std::time::{Duration, UNIX_EPOCH};

    let timestamp = time::OffsetDateTime::from_unix_timestamp_nanos(-1_500_000_000).unwrap();
    let reference = UNIX_EPOCH + Duration::from_millis(500);
    assert_eq!(
        humany::relative_time(timestamp)
            .reference(reference)
            .to_string(),
        "2 seconds ago"
    );
}
