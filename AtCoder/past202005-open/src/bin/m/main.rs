use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $a = $b;
        }
    };
}

const INF: usize = std::usize::MAX;

fn distance(n: usize, start: usize, g: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut d = vec![INF; n];
    let mut que = VecDeque::new();
    d[start] = 0;
    que.push_back(start);
    while let Some(u) = que.pop_front() {
        for &v in &g[u] {
            if d[v] == INF {
                d[v] = d[u] + 1;
                que.push_back(v);
            }
        }
    }
    for u in 0..n {
        assert_ne!(d[u], INF);
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
        s: Usize1,
        k: usize,
        t: [Usize1; k],
    };

    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }

    let mut dd = vec![vec![]; k];
    for i in 0..k {
        dd[i] = distance(n, t[i], &g);
    }
    
    let mut dp = vec![vec![INF; 1 << k]; k];
    for i in 0..k {
        dp[i][1 << i] = dd[i][s];
    }
    for bits in 1..(1 << k) {
        for last in 0..k {
            if bits >> last & 1 == 0 {
                continue;
            }
            for next in 0..k {
                if bits >> next & 1 == 1 {
                    continue;
                }
                chmin!(dp[next][bits ^ (1 << next)], dp[last][bits] + dd[last][t[next]]);
            }
        }
    }
    let mut ans = INF;
    for last in 0..k {
        chmin!(ans, dp[last][(1 << k) - 1]);
    }
    println!("{}", ans);
}
