use input_i_scanner::{scan_with, InputIScanner};

fn inv(a: i64, m: i64) -> i64 {
    let (x, _, _) = ext_gcd::ext_gcd(a, m);
    (x + m) % m
}

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let t = scan_with!(_i_i, i64);
    let m = 998244353;

    let mut ans = 0;
    for i in 1..=t {
        ans += inv(i + 1, m);
        ans %= m;
    }
    println!("{}", ans);
}
