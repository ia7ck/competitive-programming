extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut dp = 1000;
    for i in 1..n {
        let nxt = dp % a[i - 1] + dp / a[i - 1] * a[i];
        dp = std::cmp::max(dp, nxt);
    }
    println!("{}", dp);
}
