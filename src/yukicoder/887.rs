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
    let mut i1 = 0;
    let mut mx = n;
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
        i1 += 1;
        mx = std::cmp::max(mx, n);
    }
    println!("{}", i1);
    println!("{}", mx);
}
