use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            r: [usize; n],
        };

        solve(n, r);
    }
}

fn solve(n: usize, r: Vec<usize>) {
    let mut ans = vec![usize::MAX; n];
    let mut heap = BinaryHeap::new();
    for j in 0..n {
        heap.push((Reverse(r[j]), j));
    }
    while let Some((Reverse(a), j)) = heap.pop() {
        if ans[j] != usize::MAX {
            continue;
        }

        ans[j] = a;

        if j >= 1 && ans[j - 1] == usize::MAX {
            if r[j - 1] > a + 1 {
                heap.push((Reverse(a + 1), j - 1));
            }
        }

        if j + 1 < n && ans[j + 1] == usize::MAX {
            if r[j + 1] > a + 1 {
                heap.push((Reverse(a + 1), j + 1));
            }
        }
    }
    let mut total = 0;
    for j in 0..n {
        assert!(r[j] >= ans[j]);
        total += r[j] - ans[j];
    }
    println!("{}", total);
}
