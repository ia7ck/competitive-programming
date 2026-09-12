use proconio::{input, marker::Bytes};

use crate::least_prime_factors::least_prime_factors;

fn main() {
    input! {
        s: Bytes,
    };

    let lpf = least_prime_factors(9999999);
    let mut primes = Vec::with_capacity(664579);
    for i in 1..lpf.len() {
        if i == lpf[i] {
            primes.push(i);
        }
    }

    for p in primes {
        let k = keta(p);
        if k != s.len() {
            continue;
        }

        let mut ok = true;
        'ij: for i in 0..s.len() {
            for j in (i + 1)..s.len() {
                if (s[i] == s[j]) != (digit(p, k, i) == digit(p, k, j)) {
                    ok = false;
                    break 'ij;
                }
            }
        }
        if ok {
            println!("{}", p);
            return;
        }
    }

    println!("-1");
}

fn keta(x: usize) -> usize {
    assert_ne!(x, 0);
    let mut x = x;
    let mut k = 0;
    while x > 0 {
        x /= 10;
        k += 1;
    }
    k
}

fn digit(x: usize, k: usize, pos: usize) -> usize {
    // x.len() = k
    assert!(pos < k);

    let mut x = x;
    let mut pos = k - pos - 1;
    while pos > 0 {
        x /= 10;
        pos -= 1;
    }

    x % 10
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod least_prime_factors {
    pub fn least_prime_factors(n: usize) -> Vec<usize> {
        let mut result = vec![0; n + 1];
        #[allow(clippy::needless_range_loop)]
        for i in 2..=n {
            result[i] = i;
        }
        for i in 2..=n {
            if result[i] == i {
                for j in ((i + i)..=n).step_by(i) {
                    if result[j] == j {
                        result[j] = i;
                    }
                }
            }
        }
        result
    }
}
