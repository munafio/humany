use humany::{relative_secs, Humany, Locale};

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
