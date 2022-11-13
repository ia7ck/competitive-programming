use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        edges: [(Usize1, Usize1, usize); m],
        switches: [Usize1; k],
    };

    let mut switch = vec![false; n];
    for s in switches {
        switch[s] = true;
    }
    let id = |v: usize, switch_push_even: bool| -> usize { v + (switch_push_even as usize) * n };
    let mut g = vec![vec![]; n * 2];
    for (u, v, a) in edges {
        let f = a == 1;
        g[id(u, f)].push((v, f));
        g[id(v, f)].push((u, f));
    }
    let inf = std::u64::MAX;
    let mut dist = vec![inf; n * 2];
    dist[id(0, true)] = 0;
    let mut deque = VecDeque::new();
    deque.push_back((0, true));
    while let Some((v, f)) = deque.pop_front() {
        if switch[v] {
            if dist[id(v, !f)] > dist[id(v, f)] {
                dist[id(v, !f)] = dist[id(v, f)];
                deque.push_front((v, !f));
            }
        }
        for &(y, ff) in &g[id(v, f)] {
            assert_eq!(f, ff);
            if dist[id(y, f)] > dist[id(v, f)] + 1 {
                dist[id(y, f)] = dist[id(v, f)] + 1;
                deque.push_back((y, f));
            }
        }
    }
    let ans = dist[id(n - 1, false)].min(dist[id(n - 1, true)]);
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
