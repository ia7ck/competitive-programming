use proconio::{input, marker::Usize1};

macro_rules! chmin {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a = $b;
        }
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, i64); m],
    };

    const INF: i64 = i64::MAX / 3;
    let mut dist = vec![vec![INF; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for &(u, v, w) in &edges {
        dist[u][v] = w;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }
    let mut dp = vec![vec![INF; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = 0;
    }
    for visit in 0..(1 << n) {
        for cur in 0..n {
            if visit >> cur & 1 == 0 {
                continue;
            }
            if dp[visit][cur] == INF {
                continue;
            }
            for u in 0..n {
                if visit >> u & 1 == 1 {
                    continue;
                }
                if dist[cur][u] == INF {
                    continue;
                }
                chmin!(dp[visit ^ (1 << u)][u], dp[visit][cur] + dist[cur][u]);
            }
        }
    }
    let mut ans = INF;
    for i in 0..n {
        chmin!(ans, dp[(1 << n) - 1][i]);
    }
    if ans == INF {
        println!("No");
    } else {
        println!("{}", ans);
    }
}
