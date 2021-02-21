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
    let tot = s
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|x| if x == 0 { 10 } else { x })
        .sum::<u32>();
    println!("{}", tot);
}
