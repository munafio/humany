use humany::{bytes, FormatOptions, Humany, Locale};

#[test]
fn spec_table() {
    assert_eq!(format!("{}", bytes(512)), "512 B");
    assert_eq!(format!("{}", bytes(1_024)), "1 KiB");
    assert_eq!(format!("{}", bytes(1_048_576)), "1 MiB");
    assert_eq!(format!("{}", bytes(1_073_741_824)), "1 GiB");
    assert_eq!(format!("{}", bytes(1_500_000)), "1.43 MiB");
}

#[test]
fn decimal_mode() {
    assert_eq!(format!("{}", bytes(1_500_000).decimal()), "1.5 MB");
}

#[test]
fn humany_bytes() {
    assert_eq!(format!("{}", Humany::DEFAULT.bytes(1_073_741_824)), "1 GiB");
}

#[test]
fn precision_and_custom_unit_boundaries() {
    let options = FormatOptions {
        decimals: u8::MAX,
        ..FormatOptions::new()
    };
    let formatter = Humany::DEFAULT.defaults(options);
    assert_eq!(
        formatter.bytes(u64::MAX).to_string(),
        "15.999999999999999999 EiB"
    );
    assert_eq!(bytes(0).units(&[]).to_string(), "0 B");
    assert_eq!(bytes(1024).units(&[]).to_string(), "1 KiB");
    assert_eq!(bytes(1000).decimal().units(&[]).to_string(), "1 kB");
    assert_eq!(bytes(1536).locale(Locale::Fr).to_string(), "1,5 KiB");
}
