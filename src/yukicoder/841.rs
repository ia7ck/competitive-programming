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
    let t: String = read();
    let mut a = 31;
    if s == "Sun" || s == "Sat" {
        a += 1;
        if t == "Sun" || t == "Sat" {
            a += 1;
        }
    }
    println!("8/{}", a);
}
