use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
        w: [usize; n],
        a: [usize; n],
    };

    let mut g = vec![vec![]; n];
    let mut deg = vec![0; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
        if w[u] < w[v] {
            deg[v] += 1;
        } else if w[u] > w[v] {
            deg[u] += 1;
        }
    }

    let mut que = VecDeque::new();
    for i in 0..n {
        if deg[i] == 0 {
            que.push_back(i);
        }
    }
    let mut count = vec![0; n];
    while let Some(i) = que.pop_front() {
        let mut dp = vec![0; w[i]];
        for &j in &g[i] {
            if w[i] > w[j] {
                for x in (0..w[i]).rev() {
                    if x + w[j] < w[i] {
                        dp[x + w[j]] = dp[x + w[j]].max(dp[x] + count[j]);
                    }
                }
            }
        }
        let dp = dp.into_iter().max().unwrap();
        count[i] = 1 + dp;

        for &j in &g[i] {
            if w[i] < w[j] {
                assert!(deg[j] >= 1);
                deg[j] -= 1;
                if deg[j] == 0 {
                    que.push_back(j);
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        ans += a[i] * count[i];
    }
    println!("{}", ans);
}
