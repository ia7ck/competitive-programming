extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }
    for i in k..n {
        println!("{}", if a[i - k] < a[i] { "Yes" } else { "No" });
    }
}
