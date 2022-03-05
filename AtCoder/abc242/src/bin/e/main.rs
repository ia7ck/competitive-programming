use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let t = scan!(usize);
    for _ in 0..t {
        let n = scan!(usize);
        let s = scan!(String);
        solve(n, s);
    }
}

fn solve(n: usize, s: String) {
    let mo = 998244353_u64;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 26;
    for i in 2..=n {
        dp[i] = dp[i - 2] * 26 % mo;
    }
    let mut ans = 0;
    for (i, ch) in s.chars().take(n / 2).enumerate() {
        if ch == 'A' {
            continue;
        }
        let d = ch as u64 - 'A' as u64;
        assert!((i + 1) * 2 <= n);
        ans += d * dp[n - (i + 1) * 2];
        ans %= mo;
    }
    let t: String = s.chars().take(n / 2).collect();
    let u: String = t.chars().rev().collect();
    if n % 2 == 1 {
        let m = s.chars().nth(n / 2).unwrap();
        ans += m as u64 - 'A' as u64;
        ans %= mo;
        let x = format!("{}{}{}", t, m, u);
        if x <= s {
            ans += 1;
            ans %= mo;
        }
    } else {
        let x = format!("{}{}", t, u);
        if x <= s {
            ans += 1;
            ans %= mo;
        }
    }
    println!("{}", ans);
}
