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

    // b 以下が k 個以上 && b-1 以下が k 個未満
    let f = Factorial::new_checking_modulo_prime(2001, 998244353);
    let z = a.iter().filter(|&&a| a == 0).count();
    type Mint = ModInt998244353;
    let mut ans = Mint::new(0);
    for b in 1..=m {
        let leq_b = a.iter().filter(|&&a| a != 0 && a <= b).count();
        let lt_b = a.iter().filter(|&&a| a != 0 && a < b).count();
        for i in 0..=z {
            if lt_b + i < k {
                // b-1 以下が i 個
                let p = (Mint::from(b - 1) / m).pow(i as u32);
                for j in 0..=(z - i) {
                    if leq_b + i + j >= k {
                        // b が j 個
                        let q = (Mint::new(1) / m).pow(j as u32);
                        // b+1 以上が z-i-j 個
                        let r = (Mint::from(m - b) / m).pow((z - i - j) as u32);
                        ans += Mint::from(b)
                            * (p * q * r * f.factorial(z)
                                / f.factorial(i)
                                / f.factorial(j)
                                / f.factorial(z - i - j));
                    }
                }
            }
        }
    }

    println!("{}", ans.val());
}
