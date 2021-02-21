use std::io::Read;

fn read<T: std::str::FromStr>() -> T {
    let token: String = std::io::stdin()
        .bytes()
        .map(|c| c.ok().unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

fn main() {
    let a1: i64 = read();
    let _b1: i64 = read();
    let a2: i64 = read();
    let _b2: i64 = read();
    let a3: i64 = read();
    let _b3: i64 = read();
    println!(
        "{}",
        if (a1 + a2 + a3) % 2 == 0 {
            ":-)"
        } else {
            ":-("
        }
    );
}
