use mod_int::ModInt998244353;
use prime_factorization::PrimeFactorization;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
    };

    let a = a.into_iter().filter(|x| m % x == 0).collect::<Vec<_>>();
    if a.is_empty() {
        println!("0");
        return;
    }

    if m == 1 {
        let one = a.iter().filter(|&&x| x == 1).count();
        let ans = Mint::new(2).pow(one as u32) - 1;
        println!("{}", ans.val());
        return;
    }

    type Mint = ModInt998244353;

    let f = m.prime_factorization();
    let mut count = vec![0; 1 << f.len()];
    for x in a {
        let mask = calc_mask(x, &f);
        count[mask] += 1;
    }

    let mut dp = vec![Mint::new(0); 1 << f.len()];
    dp[0] = Mint::new(1);
    for mask in 0..(1 << f.len()) {
        let mut new_dp = dp.clone();
        if count[mask] >= 1 {
            let c = Mint::new(2).pow(count[mask]) - 1;
            for i in 0..(1 << f.len()) {
                new_dp[i | mask] += dp[i] * c;
            }
        }
        dp = new_dp;
    }
    let ans = dp[(1 << f.len()) - 1];
    println!("{}", ans.val());
}

fn calc_mask(x: u64, f: &Vec<(u64, u64)>) -> usize {
    let mut mask = 0;
    for (i, &(p, e)) in f.iter().enumerate() {
        if x % p.pow(e as u32) == 0 {
            mask |= 1 << i;
        }
    }
    mask
}
