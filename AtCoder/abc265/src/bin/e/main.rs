use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
        banned: [(i64, i64); m],
    };

    const MOD: u64 = 998244353;

    macro_rules! add {
        ($a: expr, $b: expr) => {
            $a += $b;
            if ($a >= MOD) {
                $a -= MOD;
            }
        };
    }

    let banned: HashSet<(i64, i64)> = banned.into_iter().collect();
    let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;
    for s in 0..n {
        for i in 0..=s {
            for j in 0..=(s - i) {
                let k = s - i - j;
                if dp[i][j][k] == 0 {
                    continue;
                }
                let (x, y) = (
                    a * i as i64 + c * j as i64 + e * k as i64,
                    b * i as i64 + d * j as i64 + f * k as i64,
                );
                if i + 1 <= n && !banned.contains(&(x + a, y + b)) {
                    add!(dp[i + 1][j][k], dp[i][j][k]);
                }
                if j + 1 <= n && !banned.contains(&(x + c, y + d)) {
                    add!(dp[i][j + 1][k], dp[i][j][k]);
                }
                if k + 1 <= n && !banned.contains(&(x + e, y + f)) {
                    add!(dp[i][j][k + 1], dp[i][j][k]);
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                if i + j + k == n {
                    add!(ans, dp[i][j][k]);
                }
            }
        }
    }
    println!("{}", ans);
}
