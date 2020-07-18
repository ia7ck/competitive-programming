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

fn gcd(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let a: i64 = read();
    let b: i64 = read();
    let g = gcd(a, b);
    let sqrt = (g as f64).sqrt();
    if sqrt.ceil() == sqrt.floor() {
        println!("Odd");
    } else {
        println!("Even");
    }
}
