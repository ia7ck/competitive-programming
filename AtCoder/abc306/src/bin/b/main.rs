use proconio::{input};

fn main() {
    input! {
        a: [u64; 64],
    };

    let mut p = 1_u64;
    let mut ans = 0;
    for i in 0..a.len() {
        ans += a[i] * p;
        if i + 1 < a.len() {
            p *= 2;
        }
    }
    println!("{}", ans);
}
