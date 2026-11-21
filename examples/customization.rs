use humany::{bytes, duration, number, Humany, Locale, RoundingMode};

fn main() {
    println!(
        "{}",
        number(1_250_000).decimals(3).trim_trailing_zeros(false)
    );
    println!("{}", bytes(1_500_000).decimal().decimals(2));
    println!("{}", duration(3665_u64).max_units(2));
    println!(
        "{}",
        Humany::new().locale(Locale::Ar).duration(3665_u64).long()
    );
    println!("{}", number(1_250f64).rounding(RoundingMode::Floor));
}
