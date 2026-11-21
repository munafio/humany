use humany::{bytes, Humany};

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
