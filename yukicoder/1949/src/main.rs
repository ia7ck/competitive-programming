use grid_search::around;
use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        y: usize,
        x: usize,
        a: [[u64; w]; h],
    };

    let mut attack = a[y - 1][x - 1];
    let mut seen = vec![vec![false; w]; h];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), y - 1, x - 1)); // 
    while let Some((Reverse(v), i, j)) = heap.pop() {
        if seen[i][j] {
            continue;
        }
        if attack > v {
            attack += v;
        } else {
            break;
        }
        seen[i][j] = true;
        for (ni, nj) in
            around(i, j)
                .y_range(0..h)
                .x_range(0..w)
                .directions(&[(-1, 0), (0, -1), (0, 1), (1, 0)])
        {
            if seen[ni][nj] {
                continue;
            }
            heap.push((Reverse(a[ni][nj]), ni, nj));
        }
    }
    let mut all = true;
    for i in 0..h {
        for j in 0..w {
            all &= seen[i][j];
        }
    }
    if all {
        println!("Yes");
    } else {
        println!("No");
    }
}
