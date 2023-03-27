use factorials::Factorial;
use mod_int::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    };

    // b-1 以下が k 個未満 && b 以下が k 個以上
    // = b-1 以下が k 個未満 - b 以下が k 個未満
    let f = Factorial::new_checking_modulo_prime(2001, 998244353);
    let z = a.iter().filter(|&&a| a == 0).count();
    type Mint = ModInt998244353;
    let mut g = vec![Mint::new(0); m + 1];
    g[0] = Mint::new(1);
    for b in 1..=m {
        let leq_b = a.iter().filter(|&&a| a != 0 && a <= b).count();
        for i in 0..=z {
            if leq_b + i < k {
                g[b] += (Mint::from(b) / m).pow(i as u32)
                    * (Mint::from(m - b) / m).pow((z - i) as u32)
                    * f.binomial(z, i);
            }
        }
    }
    let mut ans = Mint::new(0);
    for b in 1..=m {
        ans += (g[b - 1] - g[b]) * b;
    }

    println!("{}", ans.val());
}
