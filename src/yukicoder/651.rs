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
    let a: i32 = read();
    let m = 10 * 60 + a * 1000 / (100 * 1000 / 60);
    println!("{:02}:{:02}", m / 60, m % 60);
}
