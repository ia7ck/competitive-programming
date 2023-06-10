use std::{collections::BinaryHeap, cmp::Reverse};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        edges: [(Usize1, Usize1); m],
        ph: [(Usize1, i64); k],
    };

    let mut g = vec![vec![]; n];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }

    const INF: i64 = std::i64::MAX / 2;
    let mut dist = vec![INF; n];
    let mut heap = BinaryHeap::new();
    for (p, h) in ph {
        dist[p] = -h;
        heap.push(Reverse((dist[p], p)));
    }
    while let Some(Reverse((dv, v))) = heap.pop() {
        if dist[v] < dv {
            continue;
        }
        assert_eq!(dist[v], dv);
        for &x in &g[v] {
            if dist[x] > dv + 1 {
                dist[x] = dv + 1;
                heap.push(Reverse((dist[x], x)));
            }
        }
    }

    let mut ans = Vec::new();
    for v in 0..n {
        if dist[v] <= 0 {
            ans.push(v + 1);
        }
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{}", ans[i]);
        if i + 1 < ans.len() {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
