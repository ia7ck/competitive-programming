use proconio::input;

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $a = $b;
        }
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(f64, f64); n],
        pq: [(f64, f64); m],
    };

    let f = |i: usize| -> (f64, f64) {
        if i < n {
            xy[i]
        } else if i < n + m {
            pq[i - n]
        } else {
            assert_eq!(i, n + m);
            (0.0, 0.0)
        }
    };

    let mut dist = vec![vec![0.0; n + m + 1]; n + m + 1];
    for i in 0..=n + m {
        for j in 0..=n + m {
            let (x1, y1) = f(i);
            let (x2, y2) = f(j);
            dist[i][j] = (x1 - x2).hypot(y1 - y2);
            dist[j][i] = (x1 - x2).hypot(y1 - y2);
        }
    }

    let inf = 1e18;
    let mut dp = vec![vec![inf; n + m + 1]; 1 << (n + m + 1)];
    dp[1 << (n + m)][n + m] = 0.0; // (0, 0)
    for bits in 1..1 << (n + m + 1) {
        let mut cost = 1.0;
        for i in 0..m {
            if bits >> (n + i) & 1 == 1 {
                cost *= 0.5;
            }
        }
        for u in 0..=n + m {
            if bits >> u & 1 == 1 && dp[bits][u] < inf {
                for v in 0..=n + m {
                    if bits >> v & 1 == 0 {
                        chmin!(dp[bits ^ (1 << v)][v], dp[bits][u] + dist[u][v] * cost);
                    }
                }
            }
        }
    }

    let mut ans = inf;
    for bits in 1..1 << (n + m + 1) {
        let mut ok = true;
        for i in 0..n {
            if bits >> i & 1 == 0 {
                ok = false;
            }
        }
        if ok {
            let mut cost = 1.0;
            for i in 0..m {
                if bits >> (n + i) & 1 == 1 {
                    cost *= 0.5;
                }
            }
            for u in 0..=n + m {
                chmin!(ans, dp[bits][u] + dist[u][n + m] * cost);
            }
        }
    }
    assert_ne!(ans, inf);
    println!("{}", ans);
}
