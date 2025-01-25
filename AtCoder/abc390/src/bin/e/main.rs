use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: usize,
        items: [(Usize1, i64, usize); n],
    };

    let mut dp = vec![vec![-1; x + 1]; 3];
    for i in 0..3 {
        dp[i][0] = 0;
    }
    for (v, a, c) in items {
        for j in (0..=x).rev() {
            if j + c <= x {
                dp[v][j + c] = dp[v][j + c].max(dp[v][j] + a);
            }
        }
        for j in 0..x {
            dp[v][j + 1] = dp[v][j + 1].max(dp[v][j]);
        }
    }

    let mut ans = 0;
    for y in 0..=x {
        for z in 0..=(x - y) {
            ans = ans.max(dp[0][y].min(dp[1][z]).min(dp[2][x - y - z]));
        }
    }
    println!("{}", ans);
}
