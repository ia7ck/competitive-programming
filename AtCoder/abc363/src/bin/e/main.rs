use std::{cmp::Reverse, collections::BinaryHeap};

use grid_search::around;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        y: usize,
        a: [[usize; w]; h],
    };

    let mut seen = vec![vec![false; w]; h];
    let mut heap = BinaryHeap::new();
    for j in 0..w {
        seen[0][j] = true;
        seen[h - 1][j] = true;
        heap.push((Reverse(a[0][j]), 0, j));
        heap.push((Reverse(a[h - 1][j]), h - 1, j));
    }
    for i in 0..h {
        seen[i][0] = true;
        seen[i][w - 1] = true;
        heap.push((Reverse(a[i][0]), i, 0));
        heap.push((Reverse(a[i][w - 1]), i, w - 1));
    }
    let mut count = 0;
    let mut sink = vec![vec![false; w]; h];
    for y in 1..=y {
        while let Some((Reverse(x), i, j)) = heap.peek().copied() {
            if x > y {
                break;
            }
            heap.pop(); // x, i, j
            if sink[i][j] {
                continue;
            }
            sink[i][j] = true;
            count += 1;
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
                if sink[ni][nj] {
                    continue;
                }
                heap.push((Reverse(a[ni][nj]), ni, nj));
            }
        }
        assert!(count <= h * w);
        println!("{}", h * w - count);
    }
}
