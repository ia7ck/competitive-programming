use input_i_scanner::{scan_with, InputIScanner};
use mod_int::ModInt998244353;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let a = scan_with!(_i_i, usize; n);
    let b = scan_with!(_i_i, usize; n);

    type Mint = ModInt998244353;
    let m = 3000;
    let mut dp = vec![Mint::new(0); m + 1];
    dp[0] = Mint::new(1);
    for i in 0..n {
        let mut nxt = vec![Mint::new(0); m + 1];
        let mut acc = Mint::new(0);
        for j in 0..=m {
            acc += dp[j];
            if a[i] <= j && j <= b[i] {
                nxt[j] = acc;
            }
        }
        dp = nxt;
    }
    let mut ans = Mint::new(0);
    for dp in dp {
        ans += dp;
    }
    println!("{}", ans.val());
}
