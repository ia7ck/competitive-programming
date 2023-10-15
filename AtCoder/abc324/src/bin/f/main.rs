use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, f64, f64); m],
    };

    // sum(b[*]) / sum(c[*]) >= x
    // sum(b[*] - x * c[*]) >= 0
    let mut ok = 0.0;
    let mut ng = 3e9;
    for _ in 0..100 {
        let x = (ok + ng) / 2.0;
        let mut g = vec![vec![]; n];
        for &(u, v, b, c) in &edges {
            g[u].push((v, b - x * c));
        }
        let mut dp = vec![-3e9_f64; n];
        dp[0] = 0.0;
        for u in 0..n {
            for &(v, y) in &g[u] {
                dp[v] = dp[v].max(dp[u] + y);
            }
        }
        if dp[n - 1] >= 0.0 {
            ok = x;
        } else {
            ng = x;
        }
    }

    println!("{}", ok);
}
