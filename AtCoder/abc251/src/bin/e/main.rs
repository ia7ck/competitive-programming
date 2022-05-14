use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    macro_rules! chmin {
        ($a: expr, $b: expr) => {
            $a = $a.min($b);
        };
    }

    let inf = std::u64::MAX;

    // 0
    let mut dp = vec![inf; n];
    dp[0] = a[0];
    dp[1] = a[0];
    for i in 1..n {
        if i + 1 < n {
            chmin!(dp[i + 1], dp[i] + a[i]);
        }
        if i + 2 < n {
            chmin!(dp[i + 2], dp[i] + a[i + 1]);
        }
    }
    let ans1 = dp[n - 1];

    // n - 1
    let mut dp = vec![inf; n];
    dp[0] = a[n - 1];
    for i in 0..(n - 1) {
        if i + 1 < n {
            chmin!(dp[i + 1], dp[i] + a[i]);
        }
        if i + 2 < n {
            chmin!(dp[i + 2], dp[i] + a[i + 1]);
        }
    }
    let ans2 = dp[n - 2].min(dp[n - 1]);

    println!("{}", ans1.min(ans2));
}
