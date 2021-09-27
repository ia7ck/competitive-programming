use input_i_scanner::{scan_with, InputIScanner};
use mod_int::ModInt998244353;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let (n, d) = scan_with!(_i_i, (usize, usize));

    type Mint = ModInt998244353;
    let two = Mint::new(2);
    let mut ans = Mint::new(0);
    for i in 1..n {
        let h = n - i;
        let mut a = Mint::new(0);
        if h >= d {
            a += two.pow(d) * 2;
        }
        if d >= 2 {
            if d <= h {
                a += two.pow(d - 2) * (d - 1) * 2;
            } else if d <= h * 2 {
                a += two.pow(d - 2) * (h * 2 + 1 - d) * 2;
            }
        }
        a *= two.pow(i - 1);
        ans += a;
    }
    println!("{}", ans.val());
}
