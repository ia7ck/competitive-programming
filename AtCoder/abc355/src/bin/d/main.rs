use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(u32, u32); n],
    };

    lr.sort();
    let mut ans = 0;
    let mut heap = BinaryHeap::<Reverse<u32>>::new();
    for (l, r) in lr {
        while heap.len() > 0 && heap.peek().unwrap().0 < l {
            heap.pop();
        }
        ans += heap.len();
        heap.push(Reverse(r));
    }
    println!("{}", ans);
}
