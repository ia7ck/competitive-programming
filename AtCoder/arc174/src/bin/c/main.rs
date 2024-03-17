use mod_int::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
    };

    // first, second
    let mut dp1 = vec![Mint::new(0); n + 1];
    let mut dp2 = vec![Mint::new(0); n + 1];
    for i in (0..n).rev() {
        // p := (n - i) / n
        // dp1[i] = p * dp2[i + 1] + (1 - p) * (dp2[i] + 1),
        // dp2[i] = p * dp1[i + 1] + (1 - p) * dp1[i]
        // -> dp2[i] = p * dp1[i + 1] + (1 - p) * (p * dp2[i + 1] + (1 - p) * (dp2[i] + 1))
        // -> (p * 2 - p^2) * dp2[i] = p * dp1[i + 1] + (1 - p) * (p * dp2[i + 1] + (1 - p))

        let p = Mint::from(n - i) / Mint::from(n);
        let q = Mint::new(1) - p;
        dp2[i] = (p * dp1[i + 1] + q * (p * dp2[i + 1] + q)) / (p * 2 - p * p);
        dp1[i] = p * dp2[i + 1] + q * (dp2[i] + Mint::new(1));
    }

    println!("{} {}", dp1[0].val(), dp2[0].val());
}
