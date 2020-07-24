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
    let x: i64 = read();
    let y: i64 = read();
    let z: i64 = read();

    let mut ans = z;
    if x <= z {
        ans -= 1;
    }
    if y <= z {
        ans -= 1;
    }

    println!("{}", ans);
}
