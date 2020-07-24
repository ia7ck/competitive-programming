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
    let mut x: i32 = read();
    let mut y: i32 = read();
    let z: i32 = read();

    if x < y {
        std::mem::swap(&mut x, &mut y);
    }

    let mut ans = 0;
    ans += y;
    x -= y;
    if x > z {
        ans += z;
    } else {
        ans += x + (z - x) / 2;
    }
    println!("{}", ans);
}
