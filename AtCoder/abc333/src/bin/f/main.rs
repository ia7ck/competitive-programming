use mod_int::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    type Mint = ModInt998244353;
    let mut dp = vec![vec![]; n + 1];
    dp[1] = vec![
        Mint::new(0), // unused
        Mint::new(1),
    ];
    for i in 2..=n {
        dp[i] = vec![Mint::new(0); i + 1];
        // dp[i][i] = dp[i-1][i-1] / 2 + dp[i-1][i-2] / 4 + ... + dp[i-1][1] / 2^(i-1) + dp[i][i] / 2^i
        dp[i][i] = (1..i).fold(Mint::new(0), |acc, j| {
            acc + dp[i - 1][i - j] / Mint::new(2).pow(j as u32)
        }) / (Mint::new(1) - Mint::new(2).pow(i as u32).inv());
        dp[i][1] = dp[i][i] / 2;
        for j in 2..=i {
            dp[i][j] = dp[i - 1][j - 1] / 2 + dp[i][j - 1] / 2;
        }
    }

    for i in 1..=n {
        print!("{}", dp[n][i].val());
        if i < n {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
