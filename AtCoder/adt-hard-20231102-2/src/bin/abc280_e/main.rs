use mod_int::ModInt998244353;
use proconio::input;

fn main() {
    // dp[i] := dp[i - 2] * p + dp[i - 1] * (1 - p)
    input! {
        n: usize,
        p: u32,
    };

    type Mint = ModInt998244353;
    let p = Mint::from(p) / 100;
    let q = Mint::new(1) - p;
    let mut dp = vec![Mint::new(0); n + 1];
    dp[1] = Mint::new(1);
    for i in 2..=n {
        dp[i] = Mint::new(1) + dp[i - 2] * p + dp[i - 1] * q;
    }
    println!("{}", dp[n].val());
}
