use proconio::input;
use prime_factorization::PrimeFactorization;

fn check(n: usize, k: usize) -> bool {
    for (p, e) in k.prime_factorization() {
        let mut f = 0;
        let mut q = p;
        while n / q > 0 {
            f += n / q;
            q *= p;
        }
        if f < e {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        k: usize,
    };

    let mut ok = k;
    let mut ng = 1;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if check(mid, k) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
