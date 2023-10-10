use mod_int::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: [usize; n],
    };

    type Mint = ModInt998244353;
    let mut dp = vec![Mint::new(0); x + 1];
    dp[0] = Mint::new(1);
    for s in 0..x {
        for &t in &t {
            if s + t <= x {
                let a = dp[s] / n;
                dp[s + t] += a;
            }
        }
    }

    let mut ans = Mint::new(0);
    for s in 0..x {
        if s + t[0] > x {
            ans += dp[s] / n;
        }
    }
    ans += dp[x] / n;

    println!("{}", ans.val());
}
