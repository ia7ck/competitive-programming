use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        edges: [(Usize1, Usize1); m],
    };

    let mut graph = vec![vec![]; n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dp = vec![0; n];
    let mut heap = BinaryHeap::new();
    dp[0] = 1;
    heap.push((Reverse(a[0]), dp[0], 0));
    while let Some((Reverse(_), s, v)) = heap.pop() {
        if s < dp[v] {
            continue;
        }
        assert_eq!(s, dp[v]);

        for &x in &graph[v] {
            if a[v] <= a[x] {
                let new_s = s + usize::from(a[v] < a[x]);
                if new_s > dp[x] {
                    dp[x] = new_s;
                    heap.push((Reverse(a[x]), dp[x], x));
                }
            }
        }
    }
    println!("{}", dp[n - 1]);
}
