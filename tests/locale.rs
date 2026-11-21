use humany::{relative_secs, Humany, Locale};

#[test]
fn relative_two_minutes() {
    let cases = [
        (Locale::En, "2 minutes ago"),
        (Locale::Fr, "il y a 2 minutes"),
        (Locale::Es, "hace 2 minutos"),
        (Locale::De, "vor 2 Minuten"),
        (Locale::Zh, "2分钟前"),
        (Locale::Ja, "2分前"),
        (Locale::Pt, "há 2 minutos"),
        (Locale::Ru, "2 минут назад"),
        (Locale::Ar, "منذ دقيقتين"),
    ];
    for (locale, want) in cases {
        assert_eq!(
            format!("{}", relative_secs(120).locale(locale)),
            want,
            "locale={locale:?}"
        );
    }
}

#[test]
fn humany_presets() {
    assert_eq!(format!("{}", Humany::ZH.relative_secs(120)), "2分钟前");
    assert_eq!(
        format!("{}", Humany::FR.relative_secs(120)),
        "il y a 2 minutes"
    );
}
