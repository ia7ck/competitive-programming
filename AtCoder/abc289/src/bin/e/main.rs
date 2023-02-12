use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn solve(n: usize, _: usize, c: Vec<u8>, edges: Vec<(usize, usize)>) {
    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }
    const INF: u32 = std::u32::MAX;
    let mut dp = vec![vec![INF; n]; n];
    dp[0][n - 1] = 0;
    let mut que = VecDeque::new();
    que.push_back((0, n - 1));
    while let Some((x, y)) = que.pop_front() {
        for &nx in &g[x] {
            for &ny in &g[y] {
                if c[nx] != c[ny] && dp[nx][ny] == INF {
                    dp[nx][ny] = dp[x][y] + 1;
                    que.push_back((nx, ny));
                }
            }
        }
    }
    if dp[n - 1][0] == INF {
        println!("-1");
    } else {
        println!("{}", dp[n - 1][0]);
    }
}

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            c: [u8; n],
            edges: [(Usize1, Usize1); m],
        };

        solve(n, m, c, edges);
    }
}
