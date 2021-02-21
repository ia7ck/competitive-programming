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
    let d: i64 = read();
    let _x: Vec<i64> = (0..n).map(|_| read()).collect();
    let v: Vec<i64> = (0..n).map(|_| read()).collect();

    let s = v.iter().sum::<i64>();
    println!("{}", (d + s - 1) / s);
}
