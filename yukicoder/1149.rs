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
    let n: i32 = read();
    let k = if n % 2 == 1 { 1 } else { 2 };
    let x = (n + 1) / 2;
    println!("{} {}", k, x);
    loop {
        let t: i32 = read();
        if t == 0 {
            break;
        } else if t == 1 {
            break;
        } else if t == 2 {
            break;
        } else {
            let kk: i32 = read();
            let xx: i32 = read();
            if kk == 1 {
                println!("{} {}", kk, n + 1 - xx);
            } else {
                println!("{} {}", kk, n - xx);
            }
        }
    }
}
