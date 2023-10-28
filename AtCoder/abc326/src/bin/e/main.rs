use mod_int::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    type Mint = ModInt998244353;
    let mut dp = vec![Mint::new(0); n + 1];
    dp[n] = Mint::new(0);
    let mut cum_sum = Mint::new(0);
    for i in (0..n).rev() {
        // for j in (i + 1)..=n {
        //     dp[i] = dp[i] + (dp[j] + a[j - 1]) / n;
        // }
        cum_sum += (dp[i + 1] + a[i]) / n;
        dp[i] = cum_sum;
    }
    println!("{}", dp[0].val());
}
