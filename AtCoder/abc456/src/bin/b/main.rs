use proconio::input;

fn main() {
    input! {
        a: [u8; 6],
        b: [u8; 6],
        c: [u8; 6],
    };

    let total = 6_f64.powi(3);
    let mut hit = 0;
    for &a in &a {
        for &b in &b {
            for &c in &c {
                if f(a, b, c) {
                    hit += 1;
                }
            }
        }
    }

    let ans = hit as f64 / total;
    println!("{}", ans);
}

fn f(a: u8, b: u8, c: u8) -> bool {
    match (a, b, c) {
        (4, 5, 6) => true,
        (4, 6, 5) => true,
        (5, 4, 6) => true,
        (5, 6, 4) => true,
        (6, 4, 5) => true,
        (6, 5, 4) => true,
        _ => false,
    }
}
