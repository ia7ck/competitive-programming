use proconio::input;

// dp[i][j] := dp[i - 1][j] + dp[i - 1][j - x]
// -> dp[i - 1][j] = dp[i][j] - dp[i - 1][j - x]

fn main() {
    input! {
        q: usize,
        k: usize,
    };

    const M: usize = 998244353;
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for _ in 0..q {
        input! {
            op: char,
            x: usize,
        };
        if op == '+' {
            for j in (0..=k).rev() {
                if j >= x {
                    dp[j] += dp[j - x];
                    dp[j] %= M;
                }
            }
        } else {
            for j in 0..=k {
                if j >= x {
                    // dp[j] -= dp[j - x];
                    dp[j] = (M + dp[j] - dp[j - x]) % M;
                }
            }
        }
        println!("{}", dp[k]);
    }
}
