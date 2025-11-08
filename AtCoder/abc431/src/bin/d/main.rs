use proconio::input;

macro_rules! chmax {
    ($a: expr, $b: expr) => {
        $a = $a.max($b);
    };
}

fn main() {
    input! {
        n: usize,
        items: [(usize, i64, i64); n],
    };

    const M: usize = 500 * 500;
    // dp[sum(w_b) - sum(w_h)] = max(*)
    let mut dp = vec![-1; M * 2 + 1];
    dp[0 + M] = 0;
    for (w, h, b) in items {
        let mut new_dp = vec![-1; M * 2 + 1];
        for i in 0..=(M * 2) {
            if dp[i] >= 0 {
                if i + w <= M * 2 {
                    chmax!(new_dp[i + w], dp[i] + b);
                }
                // i - w >= 0
                if i >= w {
                    chmax!(new_dp[i - w], dp[i] + h);
                }
            }
        }
        dp = new_dp;
    }

    let mut ans = 0;
    for i in M..=(M * 2) {
        chmax!(ans, dp[i]);
    }
    println!("{}", ans);
}
