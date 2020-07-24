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
    let b: i32 = read();
    let c: i32 = read();
    println!(
        "{}",
        vec![a - b, b - c, a - c]
            .iter()
            .map(|x| x.abs())
            .min()
            .unwrap()
    );
}
