use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, u32); m],
    };

    let mut g = vec![vec![]; n];
    for (u, v, w) in edges {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut ans = 0;
    for b in (0..30).rev() {
        // reachability

        let mut visited = vec![false; n];
        let mut que = VecDeque::new();
        visited[0] = true;
        que.push_back(0);
        while let Some(x) = que.pop_front() {
            for &(y, w) in &g[x] {
                if visited[y] || w >> b & 1 == 1 {
                    continue;
                }
                visited[y] = true;
                que.push_back(y);
            }
        }
        if visited[n - 1] {
            for i in 0..n {
                g[i].retain(|&(_, w)| w >> b & 1 == 0);
            }
        } else {
            ans ^= 1 << b;
        }
    }
    println!("{}", ans);
}
