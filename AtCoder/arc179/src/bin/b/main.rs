use proconio::{input, marker::Usize1};

fn main() {
    input! {
        m: usize,
        n: usize,
        x: [Usize1; m],
    };

    const P: usize = 998244353;
    let mut dp = vec![0; 1 << m];
    dp[(1 << m) - 1] = 1;
    for _ in 0..n {
        let mut new_dp = vec![0; 1 << m];
        for bits in 0..(1 << m) {
            for j in 0..m {
                if bits >> j & 1 == 1 {
                    let mut new_bits = bits ^ (1 << j);
                    for k in 0..m {
                        if x[k] == j {
                            new_bits |= 1 << k;
                        }
                    }
                    new_dp[new_bits] += dp[bits];
                    new_dp[new_bits] %= P;
                }
            }
        }
        dp = new_dp;
    }

    let mut ans = 0;
    for bits in 0..(1 << m) {
        ans += dp[bits];
        ans %= P;
    }
    println!("{}", ans);
}
