use prime_factorization::PrimeFactorization;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    };

    let mut squares = Vec::new();
    for i in 2..=n {
        if i * i <= n {
            squares.push(i * i);
        }
    }
    let square_set: HashSet<usize> = squares.iter().copied().collect();
    let mut count = vec![0; n + 1];
    for (i, &s) in squares.iter().enumerate() {
        count[s] += 1;
        for &t in &squares[i..] {
            if s * t <= n && !square_set.contains(&(s * t)) {
                count[s * t] += 1;
            }
        }
    }
    for i in 1..=n {
        count[i] += count[i - 1];
    }

    let mut ans = 0_u64;
    for i in 1..=n {
        let mut j = 1;
        for (p, e) in i.prime_factorization() {
            if e % 2 == 1 {
                j *= p;
            }
        }
        ans += 1 + count[n / j];
    }
    println!("{}", ans);
}
