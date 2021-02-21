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
    let n: usize = read();
    let mut a = vec![0 as i64; n];
    for i in 0..n {
        a[i] = read();
    }
    let v: i64 = read();
    println!("{}", a.iter().sum::<i64>() - v);
}
