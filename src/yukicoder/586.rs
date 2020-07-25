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
    let p1: i32 = read();
    let p2: i32 = read();
    let n: usize = read();
    let mut r = vec![0; n];
    for i in 0..n {
        r[i] = read();
    }
    let mut set = std::collections::HashSet::new();
    let mut ans = 0;
    for x in &r {
        if !set.insert(x) {
            ans += p1 + p2;
        }
    }
    println!("{}", ans);
}
