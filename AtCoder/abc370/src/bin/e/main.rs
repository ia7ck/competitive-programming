use std::collections::HashMap;

use mod_int::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    };

    // macro_rules! add {
    //     ($a: expr, $b: expr) => {
    //         $a = $a + $b;
    //     };
    // }
    // let mut cumsum = vec![0; n + 1];
    // let mut dp = vec![Mint::new(0); n + 1];
    // dp[0] = Mint::new(1);
    // for i in 0..n {
    //     cumsum[i + 1] = cumsum[i] + a[i];
    //     for j in 0..=i {
    //         if cumsum[i + 1] - cumsum[j] != k {
    //             add!(dp[i + 1], dp[j]);
    //         }
    //     }
    // }
    // println!("{}", dp[n].val());

    let mut cumsum_a = 0;
    let mut dp = vec![Mint::new(0); n + 1];
    dp[0] = Mint::new(1);
    let mut cumsum_dp = Mint::new(1);
    let mut sum_dp_by_cumsum = HashMap::<i64, Mint>::new();
    sum_dp_by_cumsum.insert(0, Mint::new(1));
    for i in 0..n {
        cumsum_a += a[i];
        dp[i + 1] = cumsum_dp;
        if let Some(&s) = sum_dp_by_cumsum.get(&(cumsum_a - k)) {
            dp[i + 1] -= s;
        }
        cumsum_dp += dp[i + 1];
        *sum_dp_by_cumsum.entry(cumsum_a).or_insert(Mint::new(0)) += dp[i + 1];
    }

    println!("{}", dp[n].val());
}
