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
    let n: usize = read();
    let m: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();
    let b: Vec<i32> = (0..m).map(|_| read()).collect();

    let mr = match a.iter().find(|&&y| x == y) {
        Some(_) => true,
        _ => false,
    };
    let mx = match b.iter().find(|&&y| x == y) {
        Some(_) => true,
        _ => false,
    };
    println!(
        "{}",
        if mr && !mx {
            "MrMax"
        } else if !mr && mx {
            "MaxValu"
        } else if mr && mx {
            "MrMaxValu"
        } else {
            "-1"
        }
    );
}
