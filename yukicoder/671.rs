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
    let s: String = read();
    let s = s.chars().collect::<Vec<_>>();
    println!(
        "{}",
        (8 - s.iter().filter(|&&c| c == '0').count() as i32).abs()
    );
}
