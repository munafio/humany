use humany::{bytes, duration, number, percent, relative_secs, Humany};

fn main() {
    let followers = number(1_500_000);
    let storage = bytes(1_073_741_824);
    let uptime = duration(86_400_u64);

    println!("Followers: {followers}");
    println!("Storage: {storage}");
    println!("Uptime: {uptime}");
    println!("Ratio: {}", percent(0.85623).decimals(2));
    println!("Last seen: {}", relative_secs(120));

    let ar = Humany::AR;
    println!("Arabic: {}", ar.relative_secs(120));
}
