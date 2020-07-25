extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: f64,
        m: f64,
        d: i64,
    }
    println!(
        "{}",
        if d == 0 {
            1.0 / n * (m - 1.0)
        } else {
            (n - (d as f64)) * 2.0 / n / n * (m - 1.0)
        }
    );
}
