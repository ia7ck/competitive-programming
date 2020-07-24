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
    let n: i32 = read();
    let h: i32 = read();
    let t: Vec<i32> = (0..n).map(|_| read()).collect();
    println!(
        "{}",
        t.iter()
            .map(|&x| (x + h).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
