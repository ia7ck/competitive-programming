use std::{cmp::Reverse, collections::BinaryHeap};

use fenwick_tree::FenwickTree;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut g = vec![vec![]; n];
    for &(a, b) in &edges {
        g[a].push(b);
        g[b].push(a);
    }

    let mut ans = Vec::new();
    let mut push = vec![false; n];
    let mut f = FenwickTree::new(n, 0);
    let mut heap = BinaryHeap::new();
    push[0] = true;
    f.add(0, 1);
    for &x in &g[0] {
        push[x] = true;
        f.add(x, 1);
        heap.push(Reverse(x));
    }
    for k in 0..n {
        while let Some(&Reverse(v)) = heap.peek() {
            if v > k {
                break;
            }
            heap.pop(); // v
            for &x in &g[v] {
                if !push[x] {
                    push[x] = true;
                    f.add(x, 1);
                    heap.push(Reverse(x));
                }
            }
        }
        
        // push[..=k].iter().all(|&p| p)
        if f.sum(..=k) == k + 1 {
            ans.push(Some(heap.len()));
        } else {
            ans.push(None);
        }
    }

    for ans in ans {
        if let Some(x) = ans {
            println!("{}", x);
        } else {
            println!("-1");
        }
    }
}
