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
    let mut ans: i64 = 0;
    for x in 0..=n {
        for y in 0..=n {
            let z = n - x - y;
            if z >= 0 && z <= n {
                ans += 1;
            }
            // dbg!((x, y, z));
        }
    }
    println!("{}", ans);
}
