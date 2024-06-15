use factorials::Factorial;
use mod_int::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        k: usize,
        c: [usize; 26],
    };

    type Mint = ModInt998244353;

    let f = Factorial::new(1001, 998244353);

    let mut dp = vec![Mint::new(0); k + 1];
    dp[0] = Mint::new(1);
    for i in 0..26 {
        let mut new_dp = vec![Mint::new(0); k + 1];
        for l in 0..=k {
            for j in 0..=c[i].min(k) {
                if l + j <= k {
                    new_dp[l + j] += dp[l] * f.binomial(l + j, j);
                }
            }
        }
        dp = new_dp;
    }

    let mut ans = Mint::new(0);
    for i in 1..=k {
        ans += dp[i];
    }
    println!("{}", ans.val());
}
