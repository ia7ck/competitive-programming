use least_prime_factors::least_prime_factors;
use proconio::input;

// a < b < c <= 1e6
fn main() {
    input! {
        n: usize,
    };

    const M: usize = 1_000_000;
    let lpf = least_prime_factors(M + 1);
    // 2, 3, 5, 7, 11, 13, ...
    let mut primes = Vec::new();
    for p in 2..=M {
        if lpf[p] == p {
            primes.push(p);
        }
    }
    let mut le = vec![0; M + 1];
    for p in 2..=M {
        le[p] = le[p - 1];
        if lpf[p] == p {
            le[p] = p;
        }
    }
    let mut ge = vec![0; M + 1];
    if primes.contains(&M) {
        ge[M] = M;
    } else {
        ge[M] = std::usize::MAX;
    }
    for p in (2..M).rev() {
        ge[p] = ge[p + 1];
        if lpf[p] == p {
            ge[p] = p;
        }
    }
    let mut index = vec![0; M + 1];
    for i in 0..primes.len() {
        index[primes[i]] = i;
    }

    let mut ans = 0;
    for i in 0..primes.len() {
        for j in (i + 2)..primes.len() {
            let (a, c) = (primes[i], primes[j]);
            // a * a * b * c * c <= n
            if (a * a).saturating_mul(c * c) > n {
                break;
            }
            let b_min = a + 1;
            let b_max = (n / (a * a) / (c * c)).min(c - 1);
            let x = ge[b_min];
            let y = le[b_max];
            if x <= y {
                ans += index[y] - index[x] + 1;
            }
        }
    }

    println!("{}", ans);
}

// p(6) = 1/6 * p(6) + 1/6 * p(3) + 1/6 * p(2) + 1/6 * p(1)
// p(3) = 1/6 * p(3) + 1/6 * p(1)
// p(2) = 1/6 * p(2) + 1/6 * p(1)
// 
// 5/6 * p(6) = 1/6 * 1/5 + 1/6 * 1/5 + 1/6
// p(6) = (1/5 + 1/5 + 1) / 5 = 7/25
