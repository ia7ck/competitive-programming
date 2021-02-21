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
    let t: String = read();
    let t: Vec<&str> = t.split(':').collect();
    let m = (t[0].parse::<i32>().unwrap() * 60 + t[1].parse::<i32>().unwrap() + 5) % (24 * 60);
    println!("{:02}:{:02}", m / 60, m % 60);
}
