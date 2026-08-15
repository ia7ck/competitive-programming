use ac_library::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    };

    type Mint = ModInt998244353;
    let mut fact = vec![Mint::new(1); n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i;
    }
    let mut ifact = vec![Mint::new(1); n + 1];
    ifact[n] = fact[n].inv();
    for i in (1..n).rev() {
        ifact[i] = ifact[i + 1] * (i + 1);
    }

    let binom = |n: usize, k: usize| -> Mint { fact[n] * ifact[k] * ifact[n - k] };

    let mut ans = Mint::new(0);
    for &a in &a {
        ans += binom(n - 1, k - 1) * Mint::new(a).pow(2);
    }
    let mut prefix = Mint::new(0);
    for &a in &a {
        if k >= 2 {
            ans += prefix * a * 2 * binom(n - 2, k - 2);
        }
        prefix += a;
    }
    println!("{}", ans.val());
}
