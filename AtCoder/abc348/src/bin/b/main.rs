use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(i64, i64); n],
    };

    for &(px, py) in &points {
        let ans = (0..n)
            .max_by_key(|&i| {
                let (qx, qy) = points[i];
                ((px - qx).pow(2) + (py - qy).pow(2), Reverse(i))
            })
            .unwrap();
        println!("{}", ans + 1);
    }
}
