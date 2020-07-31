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
    let a: i32 = read();
    let b: i32 = read();
    let c: i32 = read();
    let d: i32 = read();

    let nl = "null";
    let tr = "tRue";
    let ans = if a != c {
        if a > c {
            nl
        } else {
            tr
        }
    } else if b != d {
        let e = b - d;
        if e == -1 || e == 2 {
            nl
        } else {
            tr
        }
    } else {
        "Draw"
    };
    println!("{}", ans);
}
