use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        h: [usize; n],
        queries: [(Usize1, Usize1); q],
    };

    let mut query_by_l = vec![vec![]; n];
    for (i, &(l, r)) in queries.iter().enumerate() {
        query_by_l[l].push((i, r));
    }

    let mut ans = vec![0; q];
    let mut heap = BinaryHeap::new();
    let mut seg = SegmentTree::new(n, 0, |a, b| a + b);
    for l in (0..n).rev() {
        for &(i, r) in &query_by_l[l] {
            ans[i] = seg.fold((r + 1)..);
        }
        while let Some(&(Reverse(x), i)) = heap.peek() {
            assert_eq!(x, h[i]);
            if x < h[l] {
                seg.update(i, |v| v - 1);
                heap.pop(); // x
            } else {
                break;
            }
        }
        seg.update(l, |v| v + 1);
        heap.push((Reverse(h[l]), l));
    }

    for ans in ans {
        println!("{}", ans);
    }
}
