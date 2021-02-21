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
    let n: i64 = read();
    let _k: i64 = read();
    let mut p: f64 = 0.0;
    for i in 0..=n {
        p += i as f64 / (n + 1) as f64;
    }
    println!("{}", p);
}
