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
    let mut n: i32 = read();
    for _ in 0..99 {
        let mut s = 0;
        while n > 0 {
            s += n % 10;
            n = n / 10;
        }
        n = s;
    }
    println!("{}", n);
}
