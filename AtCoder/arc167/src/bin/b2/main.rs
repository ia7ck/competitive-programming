use num_bigint::BigUint;
use proconio::input;

fn prime_factorization(mut a: u64) -> Vec<(u64, u32)> {
    let mut result = Vec::new();
    for p in 2.. {
        if p * p > a {
            break;
        }
        let mut e = 0;
        while a % p == 0 {
            a /= p;
            e += 1;
        }
        if e >= 1 {
            result.push((p, e));
        }
    }
    if a > 1 {
        result.push((a, 1));
    }
    result
}

fn main() {
    input! {
        a: u64,
        b: u64,
    };

    let px = prime_factorization(a);
    let mut ans = Option::<BigUint>::None;
    for &(p, x) in &px {
        let h = px
            .iter()
            .map(|&(q, y)| {
                if p == q {
                    BigUint::from(x) * b * (BigUint::from(x) * b + 1_u32) / 2_u32
                } else {
                    BigUint::from(y) * b + 1_u32
                }
            })
            .product::<BigUint>();
        let h = h / x;
        ans = match ans.take() {
            None => Some(h),
            Some(g) => Some(g.min(h)),
        };
    }

    let ans = ans.unwrap();
    println!("{}", ans % 998244353_u64);
}
