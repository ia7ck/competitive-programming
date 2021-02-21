extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        pts: [(i64, i64); n],
    }

    println!(
        "{}",
        pts.iter()
            .filter(|(x, y)| x * x + y * y <= d * d)
            .collect::<Vec<_>>()
            .len()
    );
}
