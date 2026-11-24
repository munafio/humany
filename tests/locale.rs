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
        (Locale::Ru, "2 минуты назад"),
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

#[test]
fn relative_direction_and_plural_forms() {
    let cases = [
        (Locale::En, -60, "in 1 minute"),
        (Locale::Fr, -60, "dans 1 minute"),
        (Locale::Ru, -60, "через 1 минуту"),
        (Locale::Ru, 12 * 60, "12 минут назад"),
        (Locale::Ru, 21 * 60, "21 минуту назад"),
        (Locale::Ru, 24 * 60, "24 минуты назад"),
        (Locale::Ar, 3 * 3600, "منذ 3 ساعات"),
        (Locale::Ar, -11 * 60, "خلال 11 دقيقة"),
    ];
    for (locale, seconds, text) in cases {
        assert_eq!(relative_secs(seconds).locale(locale).to_string(), text);
    }

    assert_eq!(
        Humany::FR.relative_secs(120).compact().to_string(),
        "il y a 2m"
    );
    assert_eq!(
        Humany::AR.relative_secs(120).compact().to_string(),
        "منذ 2m"
    );
    assert_eq!(Humany::JA.relative_secs(0).compact().to_string(), "0秒前");
}
