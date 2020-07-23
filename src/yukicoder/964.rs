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
    let mut ans = String::new();
    for i in (0..=9).rev() {
        for _ in 0..n {
            if ans.len() < n * n {
                ans.push_str(&i.to_string());
            }
        }
    }
    println!("{}", ans);
}
