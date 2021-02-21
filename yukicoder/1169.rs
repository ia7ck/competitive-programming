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
    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        for k in 0..n {
            a[(i + k) % n][(n + i - k) % n] = i + 1;
        }
    }
    for r in a {
        println!(
            "{}",
            r.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
