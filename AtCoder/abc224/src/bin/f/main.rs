use input_i_scanner::{scan_with, InputIScanner};
use mod_int::ModInt998244353;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let s = scan_with!(_i_i, String);
    let n = s.len();

    type Mint = ModInt998244353;
    let mut pow2 = vec![Mint::new(1); n + 1];
    let mut pow10 = vec![Mint::new(1); n + 1];
    for i in 1..=n {
        pow2[i] = pow2[i - 1] * 2;
        pow10[i] = pow10[i - 1] * 10;
    }
    let mut right = pow10[n - 1] * pow2[0];
    for i in 0..(n - 1) {
        right += pow10[n - i - 2] * pow2[i];
    }
    let mut ans = Mint::new(0);
    for (i, c) in s.chars().enumerate() {
        let left = pow2[i];
        let d = c as i64 - '0' as i64;
        ans += left * d * right;
        right -= pow2[(n - i - 1).saturating_sub(1)];
        right /= 10;
    }
    println!("{}", ans.val());
}
