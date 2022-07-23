use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [i64; n],
        cy: [(usize, i64); m],
    };

    let mut bonus = vec![0; n + 1];
    for (c, y) in cy {
        bonus[c] = y;
    }
    let mut dp = vec![-1; n + 1];
    dp[0] = 0;
    for x in x {
        let mut nxt_dp = vec![-1; n + 1];
        for j in 0..=n {
            if dp[j] < 0 {
                continue;
            }
            if j + 1 <= n {
                nxt_dp[j + 1] = nxt_dp[j + 1].max(dp[j] + x + bonus[j + 1]);
            }
            nxt_dp[0] = nxt_dp[0].max(dp[j]);
        }
        dp = nxt_dp;
    }
    let ans = dp.iter().max().copied().unwrap();
    assert!(ans >= 0);
    println!("{}", ans);
}
