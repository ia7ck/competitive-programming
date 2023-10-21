use std::{collections::{BTreeMap, BinaryHeap}, cmp::Reverse};

use proconio::input;

fn main() {
    input! {
        n: usize,
        td: [(u64, u64); n],
    };

    let mut ends = BTreeMap::new();
    for &(t, d) in &td {
        ends.entry(t).or_insert(Vec::new()).push(t + d);
    }
    let mut heap = BinaryHeap::new();
    let mut x = 0;
    let mut ans = 0;
    loop {
        if heap.is_empty() {
            if let Some((&y, _)) = ends.range(x..).next() {
                x = y;
            } else {
                break;
            }
        }
        if let Some(e) = ends.get(&x) {
            for &e in e {
                heap.push(Reverse(e));
            }
        }
        while let Some(Reverse(y)) = heap.pop() {
            if x <= y {
                ans += 1;
                x += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}
