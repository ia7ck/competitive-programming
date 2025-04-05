use proconio::input;

use least_prime_factors::least_prime_factors;

fn main() {
    let lpf = least_prime_factors(1_000_000 + 1);
    let mut primes = Vec::new();
    for i in 2..=1_000_000 {
        if lpf[i] == i {
            primes.push(i as u64);
        }
    }
    const M: u64 = 10_u64.pow(12);
    let mut numbers = Vec::new();
    for (i, &p) in primes.iter().enumerate() {
        for x in (2..).step_by(2) {
            let Some(px) = p.checked_pow(x) else {
                break;
            };
            if px > M {
                break;
            }
            for &q in &primes[(i + 1)..] {
                if px.checked_mul(q.pow(2)).is_none() {
                    break;
                }
                for y in (2..).step_by(2) {
                    let Some(qy) = q.checked_pow(y) else {
                        break;
                    };
                    if qy > M {
                        break;
                    }
                    match px.checked_mul(qy) {
                        Some(v) if v <= M => {
                            numbers.push(v);
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
        }
    }

    numbers.sort_unstable();
    numbers.dedup();

    input! {
        q: usize,
        queries: [u64; q],
    }

    for a in queries {
        let p = numbers.partition_point(|&x| x <= a);
        assert!(p >= 1);
        let ans = numbers[p - 1];
        println!("{}", ans);
    }
}
