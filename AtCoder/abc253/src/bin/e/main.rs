use mod_int::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };

    type Mint = ModInt998244353;

    let mut dp = vec![Mint::new(0); m + 1];
    for i in 1..=m {
        dp[i] = Mint::new(1);
    }
    for _ in 1..n {
        let mut nxt = vec![Mint::new(0); m + 1];
        for i in 1..=m {
            // i - k >= 1
            if i >= k + 1 {
                nxt[1] += dp[i];
                // i - k + 1 <= m
                if i + 1 <= m + k {
                    nxt[i - k + 1] -= dp[i];
                }
            }
            if 1 < i + k && i + k <= m {
                nxt[i + k] += dp[i];
            }
        }
        for i in 2..=m {
            nxt[i] = nxt[i] + nxt[i - 1];
        }
        dp = nxt;
    }

    let mut ans = Mint::new(0);
    for i in 1..=m {
        ans += dp[i];
    }
    println!("{}", ans.val());
}
