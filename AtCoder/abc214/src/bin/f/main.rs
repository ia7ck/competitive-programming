use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s: Vec<char> = rd.get_chars();
    let n = s.len();
    if n == 1 {
        println!("1");
        return;
    }

    let mo: u64 = 1_000_000_000 + 7;
    let mut dp = vec![0; n];
    dp[0] = 1;
    if s[0] != s[1] {
        dp[1] = 1;
    }
    for i in 2..n {
        if let Some(j) = (0..i).rev().find(|&j| s[j] == s[i]) {
            for k in j.saturating_sub(1)..=(i - 2) {
                dp[i] += dp[k];
                dp[i] %= mo;
            }
        } else {
            dp[i] = 1;
            for k in 0..=(i - 2) {
                dp[i] += dp[k];
                dp[i] %= mo;
            }
        }
    }
    let mut ans = 0;
    for dp in dp {
        ans += dp;
        ans %= mo;
    }
    println!("{}", ans);
}
