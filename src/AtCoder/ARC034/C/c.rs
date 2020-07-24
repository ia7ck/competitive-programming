extern crate proconio;
use proconio::input;

fn main() {
    input! {
      a: i64,
      b: i64,
    }
    let mut h = std::collections::HashMap::<i64, i64>::new();
    for mut x in (b + 1)..=a {
        let mut y = 2;
        while y * y <= x {
            while x % y == 0 {
                let c = h.entry(y).or_insert(0);
                *c += 1;
                x = x / y;
            }
            y += 1;
        }
        if x > 1 {
            let c = h.entry(x).or_insert(0);
            *c += 1;
        }
    }
    let mo = 1_000_000_000 + 7;
    let mut ans = 1 as i64;
    for c in h.values() {
        ans = ans * (c + 1) % mo;
    }
    println!("{}", ans);
}
