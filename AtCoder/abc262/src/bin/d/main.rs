use mod_int::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

fn solve(n: usize, a: &[usize]) -> Mint {
    let mut dp = vec![vec![Mint::new(0); n]; n + 1];
    dp[0][0] = Mint::new(1);
    for &x in a {
        let mut next_dp = dp.clone();
        for i in 0..n {
            for j in 0..n {
                next_dp[i + 1][(j + x) % n] += dp[i][j];
            }
        }
        dp = next_dp;
    }
    dp[n][0]
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = Mint::new(0);
    for n in 1..=n {
        let y = solve(n, &a);
        ans += y;
    }
    println!("{}", ans.val());
}
