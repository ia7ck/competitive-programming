extern crate proconio;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ijv: [(usize, usize, i64); n],
    }
    let mut a = vec![vec![0; w]; h];
    for (i, j, v) in ijv {
        a[i - 1][j - 1] = v;
    }
    let mut dp = vec![vec![0; 4]; w];
    let mut ans = 0;
    for i in 0..h {
        let mut next = vec![vec![0; 4]; w];
        for j in 0..w {
            for k in 0..=3 {
                // println!("{} {} {}", i, j, k);
                if j + 1 < w {
                    dp[j + 1][k] = std::cmp::max(dp[j + 1][k], dp[j][k]);
                    if a[i][j] > 0 && k < 3 {
                        dp[j + 1][k + 1] = std::cmp::max(dp[j + 1][k + 1], dp[j][k] + a[i][j]);
                    }
                }
                next[j][0] = std::cmp::max(next[j][0], dp[j][k]);
                if a[i][j] > 0 && k < 3 {
                    next[j][0] = std::cmp::max(next[j][0], dp[j][k] + a[i][j]);
                }
                ans = std::cmp::max(ans, dp[j][k]);
                ans = std::cmp::max(ans, next[j][0]);
            }
        }
        dp = next;
    }
    println!("{}", ans);
}
