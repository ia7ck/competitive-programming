use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(f64, f64); n],
    };

    const INF: f64 = 1e18;
    let mut dp = vec![vec![INF; 30]; n];
    dp[0][0] = 0.0;
    for i in 1..n {
        let (x, y) = points[i];
        for j in 0..30 {
            for k in i.saturating_sub(30)..i {
                let nj = j + (i - k - 1);
                if nj >= 30 {
                    continue;
                }
                let (xx, yy) = points[k];
                let d = (x - xx).hypot(y - yy);
                dp[i][nj] = dp[i][nj].min(dp[k][j] + d);
            }
        }
    }

    let mut ans = dp[n - 1][0];
    for j in 1..30 {
        ans = ans.min(dp[n - 1][j] + 2_f64.powi((j - 1) as i32));
    }
    println!("{}", ans);
}
