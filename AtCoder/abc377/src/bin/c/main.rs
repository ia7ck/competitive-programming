use std::collections::HashSet;

use grid_search::around;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut bad = HashSet::new();
    for (a, b) in ab {
        bad.insert((a, b));
        for (a2, b2) in around(a, b).y_range(0..n).x_range(0..n).directions(&[
            (2, 1),
            (1, 2),
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (-1, -2),
            (1, -2),
            (2, -1),
        ]) {
            bad.insert((a2, b2));
        }
    }

    let ans = n * n - bad.len();
    println!("{}", ans);
}
