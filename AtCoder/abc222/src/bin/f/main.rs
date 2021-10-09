use input_i_scanner::{scan_with, InputIScanner};
use std::collections::VecDeque;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let mut g = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let (a, b, c) = scan_with!(_i_i, (usize, usize, u64));
        g[a - 1].push((b - 1, c));
        g[b - 1].push((a - 1, c));
    }
    let d = scan_with!(_i_i, u64; n);
    let mut dp = vec![0; n];
    dfs(0, !0, &g, &mut dp, &d);
    let mut ep = vec![0; n];
    let mut que = VecDeque::new();
    que.push_back((0, !0));
    while let Some((u, p)) = que.pop_front() {
        let edges: Vec<(usize, u64)> = g[u].iter().copied().filter(|&(v, _)| v != p).collect();
        let mut left = vec![0; edges.len()];
        for (i, &(v, c)) in edges.iter().enumerate() {
            left[i] = c + dp[v].max(d[v]);
        }
        for i in 1..edges.len() {
            left[i] = left[i].max(left[i - 1]);
        }
        let mut right = 0;
        for (i, &(v, c)) in edges.iter().enumerate().rev() {
            ep[v] = d[u].max(ep[u]);
            if i >= 1 {
                ep[v] = ep[v].max(left[i - 1]);
            }
            ep[v] = ep[v].max(right);
            ep[v] += c;
            que.push_back((v, u));
            right = right.max(c + dp[v].max(d[v]));
        }
    }
    for i in 0..n {
        let ans = dp[i].max(ep[i]);
        assert!(ans >= 1);
        println!("{}", ans);
    }
}

fn dfs(u: usize, p: usize, g: &Vec<Vec<(usize, u64)>>, dp: &mut Vec<u64>, d: &Vec<u64>) {
    for &(v, c) in &g[u] {
        if v == p {
            continue;
        }
        dfs(v, u, g, dp, d);
        dp[u] = dp[u].max(c + dp[v].max(d[v]));
    }
}
