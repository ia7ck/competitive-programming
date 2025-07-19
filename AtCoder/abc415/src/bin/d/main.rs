use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: u64,
        m: usize,
        ab: [(u64, u64); m],
    };

    let mut heap = BinaryHeap::new();
    for (a, b) in ab {
        heap.push((b, Reverse(a)));
    }

    let mut ans = 0;
    let mut n = n;
    while let Some((b, Reverse(a))) = heap.pop() {
        while n >= a {
            ans += 1;
            n -= a;
            n += b;
        }
    }

    println!("{}", ans);
}
