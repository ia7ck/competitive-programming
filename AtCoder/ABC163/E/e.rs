extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut a = a.iter().zip(1..=(n as i64)).collect::<Vec<_>>();
    a.sort_by(|x, y| y.0.cmp(&x.0));
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            if i + j < n {
                let v = a[i + j].0;
                let k = a[i + j].1;
                // k -> i + 1
                dp[i + 1][j] =
                    std::cmp::max(dp[i + 1][j], dp[i][j] + v * (k - (i + 1) as i64).abs());
                // k -> n - j
                dp[i][j + 1] =
                    std::cmp::max(dp[i][j + 1], dp[i][j] + v * (k - (n - j) as i64).abs());
            }
        }
    }
    let mut ans = 0;
    for i in 0..=n {
        ans = std::cmp::max(ans, dp[i][n - i]);
    }
    println!("{}", ans);
}
