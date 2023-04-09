use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [u64; n],
    };

    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    for &x in &a {
        if seen.contains(&x) {
            continue;
        }
        seen.insert(x);
        heap.push(Reverse(x));
    }

    while k >= 2 {
        let Reverse(s) = heap.pop().unwrap();
        k -= 1;
        for &x in &a {
            let y = s + x;
            if seen.contains(&y) {
                continue;
            }
            seen.insert(y);
            heap.push(Reverse(y));
        }
    }

    let Reverse(ans) = heap.peek().unwrap();
    println!("{}", ans);
}
