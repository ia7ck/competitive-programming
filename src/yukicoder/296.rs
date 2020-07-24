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
    let h: i64 = read();
    let m: i64 = read();
    let t: i64 = read();

    let s = h * 60 + m + t * (n - 1);
    let hh = s / 60 % 24;
    let mm = s % 60;
    println!("{}\n{}", hh, mm);
}
