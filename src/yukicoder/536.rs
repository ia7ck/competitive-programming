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
    let mut s: String = read();
    // let s: Vec<char> = s.chars().collect();
    if &s[s.len() - 2..] == "ai" {
        s.replace_range(s.len() - 2.., "AI");
        println!("{}", s);
    } else {
        println!("{}-AI", s);
    }
}
