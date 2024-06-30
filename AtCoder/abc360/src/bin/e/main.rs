use proconio::input;

use mod_int::ModInt998244353;
type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    if n == 1 {
        println!("1");
        return;
    }

    // let mut dp = vec![Mint::new(0); n];
    // dp[0] = Mint::new(1);
    // for _ in 0..k {
    //     let mut new_dp = vec![Mint::new(0); n];
    //     for i in 0..n {
    //         for j in 0..n {
    //             if i == j {
    //                 new_dp[j] += dp[i] * (Mint::from(n - 1).pow(2) + 1) / Mint::from(n).pow(2);
    //             } else {
    //                 new_dp[j] += dp[i] * Mint::from(2) / Mint::from(n).pow(2);
    //             }
    //         }
    //     }
    //     dp = new_dp;
    // }

    // let mut ans = Mint::new(0);
    // for i in 0..n {
    //     ans += dp[i] * (i + 1);
    // }
    // println!("{}", ans.val());

    let n = Mint::from(n);
    let mut dp0 = Mint::new(1);
    let mut dp1 = Mint::new(0);
    for _ in 0..k {
        let new_dp0 =
            dp0 * (Mint::from(n - 1).pow(2) + 1) / n.pow(2) + dp1 * 2 / n.pow(2) * (n - 1);
        let new_dp1 = dp0 * 2 / n.pow(2)
            + dp1 * (Mint::from(n - 1).pow(2) + 1) / n.pow(2)
            + dp1 * 2 / n.pow(2) * (n - 2);
        dp0 = new_dp0;
        dp1 = new_dp1;
    }

    // dp0 * 1 + dp1 * (2 + 3 + ... + n)
    let ans = dp0 + dp1 * (n.pow(2) + n - 2) / 2;
    println!("{}", ans.val());
}
