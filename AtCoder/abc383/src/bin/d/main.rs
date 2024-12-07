use proconio::input;
use least_prime_factors::least_prime_factors;

fn main() {
    input! {
        n: usize,
    };

    let lpf = least_prime_factors(2_000_000 + 1);
    let mut primes = Vec::new();
    for x in 2.. {
        if x * x > n {
            break;
        }
        if lpf[x] == x {
            primes.push(x);
        }
    }

    // p^2 * q^2
    let mut v2 = Vec::new();
    for (i, p) in primes.iter().enumerate() {
        // q <= sqrt(n) / p 
        for q in &primes[(i + 1)..] {
            let v = p.pow(2).saturating_mul(q.pow(2));
            if v <= n {
                v2.push(v);
            } else {
                break;
            }
        }
    }

    // p^8
    let mut v8 = Vec::new();
    for p in &primes {
        let v = p.saturating_pow(8);
        if v <= n {
            v8.push(v);
        } else {
            break;
        }
    }

    let mut ans = Vec::new();
    ans.extend(v2);
    ans.extend(v8);
    ans.sort();
    ans.dedup();
    println!("{}", ans.len());
}
