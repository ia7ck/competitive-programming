use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, u64); m],
    };

    let mut g = vec![vec![]; n];
    for &(u, v, w) in &edges {
        g[u].push((v, w));
        g[v].push((u, w));
    }

    let mut dp = vec![vec![None; n]; 1 << n];
    for v in 0..n {
        dp[1 << v][v] = Some(0);
    }
    for bits in 0..(1 << n) {
        for u in 0..n {
            if let Some(d) = dp[bits][u] {
                for &(v, w) in &g[u] {
                    if bits >> v & 1 == 1 {
                        continue;
                    }
                    let next = bits ^ (1 << v);
                    dp[next][v] = dp[next][v].max(Some(d + w));
                }
            }
        }
    }

    let mut ans = 0;
    for bits in 0..(1 << n) {
        for v in 0..n {
            if let Some(d) = dp[bits][v] {
                if bits >> v & 1 == 1 {
                    ans = ans.max(d);
                }
            }
        }
    }
    println!("{}", ans);
}
