use proconio::input;

use mod_int::ModInt998244353;
type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };

    let mut dp = vec![Mint::new(0); n + 1];
    dp[0] = Mint::new(1);
    let mut ans = Mint::new(0);
    for _ in 0..k {
        let mut next = vec![Mint::new(0); n + 1];
        for i in 0..n {
            for j in 1..=m {
                if i + j < n {
                    next[i + j] += dp[i] / m;
                } else if i + j == n {
                    ans += dp[i] / m;
                } else {
                    let back = i + j - n;
                    next[n - back] += dp[i] / m;
                }
            }
        }
        dp = next;
    }

    println!("{}", ans.val());
}
