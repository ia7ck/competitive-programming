use std::{cmp::Reverse, collections::BinaryHeap};

use grid_search::around;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: u64,
        p: Usize1,
        q: Usize1,
        s: [[u64; w]; h],
    };

    let mut ans = s[p][q];
    let mut seen = vec![vec![false; w]; h];
    let mut adjacent = BinaryHeap::new();
    seen[p][q] = true;
    for (ni, nj) in
        around(p, q)
            .y_range(0..h)
            .x_range(0..w)
            .directions(&[(-1, 0), (0, -1), (1, 0), (0, 1)])
    {
        seen[ni][nj] = true;
        adjacent.push((Reverse(s[ni][nj]), (ni, nj)));
    }

    while let Some((Reverse(t), (i, j))) = adjacent.pop() {
        // t < ans / x
        if t.saturating_mul(x) < ans {
            ans += t;
            for (ni, nj) in around(i, j).y_range(0..h).x_range(0..w).directions(&[
                (-1, 0),
                (0, -1),
                (1, 0),
                (0, 1),
            ]) {
                if seen[ni][nj] {
                    continue;
                }
                seen[ni][nj] = true;
                adjacent.push((Reverse(s[ni][nj]), (ni, nj)));
            }
        } else {
            break;
        }
    }

    println!("{}", ans);
}
