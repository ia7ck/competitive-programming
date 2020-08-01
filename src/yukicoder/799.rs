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
    let mut a: i32 = read();
    let mut b: i32 = read();
    let mut c: i32 = read();
    let mut d: i32 = read();

    if a > c {
        std::mem::swap(&mut a, &mut c);
        std::mem::swap(&mut b, &mut d);
    }
    println!(
        "{}",
        if d <= b {
            (d - c + 1) * (b - a + 1 - 1)
        } else {
            (b - a + 1) * (d - c + 1) - std::cmp::max(0, b - c + 1)
        }
    );
}
