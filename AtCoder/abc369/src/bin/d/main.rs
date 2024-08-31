use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    let mut dp = vec![Option::<u64>::None; 2];
    dp[0] = Some(0);
    for x in a {
        let mut new_dp = vec![Option::<u64>::None; 2];
        if let Some(dp) = dp[0] {
            new_dp[0] = match new_dp[0].take() {
                Some(v) => Some(v.max(x).max(dp)),
                None => Some(dp),
            };
            new_dp[1] = match new_dp[1].take() {
                Some(v) => Some(v.max(dp + x)),
                None => Some(dp + x),
            };
        }
        if let Some(dp) = dp[1] {
            new_dp[0] = match new_dp[0].take() {
                Some(v) => Some(v.max(dp + x + x)),
                None => Some(dp + x + x),
            };
            new_dp[1] = match new_dp[1].take() {
                Some(v) => Some(v.max(x).max(dp)),
                None => Some(dp),
            };
        }
        dp = new_dp;
    }

    let mut ans = 0;
    for dp in dp {
        if let Some(v) = dp {
            ans = ans.max(v);
        }
    }
    assert_ne!(ans, u64::MAX);
    println!("{}", ans);
}
