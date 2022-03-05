use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let n = scan!(usize);

    let mo = 998244353_u64;
    let mut dp = vec![0; 10];
    for d in 1..=9 {
        dp[d] = 1;
    }
    for _ in 0..(n - 1) {
        let mut nxt = vec![0; 10];
        for a in 1..=9 {
            for b in 1..=9 {
                if a.max(b) - a.min(b) <= 1 {
                    nxt[b] += dp[a];
                    nxt[b] %= mo;
                }
            }
        }
        dp = nxt;
    }
    let mut ans = 0;
    for &dp in &dp[1..] {
        ans += dp;
        ans %= mo;
    }
    println!("{}", ans);
}
