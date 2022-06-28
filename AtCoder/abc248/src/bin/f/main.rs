use join::Join;
use mod_int::{DynamicModulo, ModInt};
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u32,
    };

    DynamicModulo::set(p as i64);

    type Mint = ModInt<DynamicModulo>;
    let mut dp = vec![Mint::new(0); n];
    let mut ep = vec![Mint::new(0); n];
    dp[0] = Mint::new(1);
    ep[1] = Mint::new(1);
    for _ in 2..=n {
        let mut nxt_dp = vec![Mint::new(0); n];
        let mut nxt_ep = vec![Mint::new(0); n];
        for j in 0..n {
            if j + 2 < n {
                // +---+
                //
                // +   +
                //
                //
                // +   +
                //
                // +---+
                nxt_ep[j + 2] += dp[j] * 2;
            }
            if j + 1 < n {
                // +---+
                //
                // +---+
                //
                //
                // +---+
                //     |
                // +   +
                //
                //
                // +   +
                //     |
                // +---+
                nxt_dp[j + 1] += dp[j] * 3;

                // +---+
                //
                // +---+
                nxt_ep[j + 1] += ep[j];
            }

            // +---+
            //     |
            // +---+
            nxt_dp[j] += dp[j] + ep[j];
        }
        dp = nxt_dp;
        ep = nxt_ep;
    }

    println!("{}", dp[1..].iter().map(|x| x.val()).join(" "));
}
