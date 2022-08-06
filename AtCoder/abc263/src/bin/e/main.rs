use mod_int::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
    };

    type Mint = ModInt998244353;
    let mut dp = vec![Mint::new(0); n + 1];
    let mut cum_sum = vec![Mint::new(0); n + 2];
    for i in (1..=(n - 1)).rev() {
        let s = cum_sum[i + 1] - cum_sum[i + 1 + a[i - 1]];
        dp[i] = (s + a[i - 1] + 1) / Mint::from(a[i - 1]);
        cum_sum[i] = cum_sum[i + 1] + dp[i];
    }

    println!("{}", dp[1].val());
}
