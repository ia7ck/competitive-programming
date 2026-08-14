use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[u64; m]; n],
    };

    let mut dp = vec![vec![0; m]; n];
    dp[0][0] = a[0][0];
    for i in 0..n {
        for j in 0..m {
            if i + 1 < n {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] + a[i + 1][j]);
            }
            if j + 1 < m {
                dp[i][j + 1] = dp[i][j + 1].max(dp[i][j] + a[i][j + 1]);
            }
        }
    }

    println!("{}", dp[n - 1][m - 1]);
}
