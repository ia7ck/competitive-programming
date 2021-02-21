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
    let x: i32 = read();
    println!(
        "{}",
        if x == 0 || x == 4 || x == 10 {
            "Yes"
        } else {
            "No"
        }
    );
}
