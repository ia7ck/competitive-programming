use std::collections::HashMap;

use prime_factorization::PrimeFactorization;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut ans = 0_usize;
    let mut count = HashMap::new();
    let mut z = 0;
    for (i, x) in a.into_iter().enumerate() {
        if x == 0 {
            ans += i;
            z += 1;
        } else {
            let mut ps = Vec::new();
            for (p, e) in x.prime_factorization() {
                if e % 2 == 1 {
                    ps.push(p);
                }
            }
            ans += z + count.get(&ps).unwrap_or(&0);
            *count.entry(ps).or_insert(0) += 1;
        }
    }
    println!("{}", ans);
}
