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
    println!(
        "{}",
        s.replace("I", "1")
            .replace("l", "1")
            .replace("O", "0")
            .replace("o", "0")
    );
}
