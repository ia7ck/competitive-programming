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
    let mut v = vec![(2, 8), (3, 9), (7, 9)];
    for _ in 0..n {
        let x1: i32 = read();
        let y1: i32 = read();
        let x2: i32 = read();
        let y2: i32 = read();
        for (x, y) in &mut v {
            if (*x, *y) == (x1, y1) {
                *x = x2;
                *y = y2;
            }
        }
    }
    println!(
        "{}",
        if v == vec![(5, 8), (4, 8), (6, 8)] {
            "YES"
        } else {
            "NO"
        }
    );
}
