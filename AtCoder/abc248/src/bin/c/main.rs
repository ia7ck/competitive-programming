use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };

    let mo = 998244353_u64;
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for _ in 0..n {
        let mut nxt = vec![0; k + 1];
        for s in 0..=k {
            for x in 1..=m {
                if s + x <= k {
                    nxt[s + x] += dp[s];
                    nxt[s + x] %= mo;
                }
            }
        }
        dp = nxt;
    }
    let mut ans = 0;
    for s in 0..=k {
        ans += dp[s];
        ans %= mo;
    }
    println!("{}", ans);
}
