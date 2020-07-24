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
    let m: usize = read();
    let op: char = read();
    let b: Vec<i64> = (0..m).map(|_| read()).collect();
    let a: Vec<i64> = (0..n).map(|_| read()).collect();
    for i in 0..n {
        println!(
            "{}",
            b.iter()
                .map(|x| if op == '+' { x + a[i] } else { x * a[i] })
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
