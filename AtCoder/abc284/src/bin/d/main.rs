use proconio::input;

fn square_root(x: u64) -> Option<u64> {
    // lb * lb <= x
    let mut lb = 1;
    let mut ub = 3_000_000_000;
    while ub - lb > 1 {
        let mid = (ub + lb) / 2;
        if mid * mid <= x {
            lb = mid;
        } else {
            ub = mid;
        }
    }
    if lb * lb == x {
        Some(lb)
    } else {
        None
    }
}

fn main() {
    const K: usize = 3_000_000;
    let mut is_prime = [true; K + 1];
    is_prime[1] = false;
    for i in 2..=K {
        if is_prime[i] {
            for j in ((i + i)..=K).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: u64,
        };

        let mut found = false;
        for x in 2..=K {
            if is_prime[x] {
                let x = x as u64;

                // p = x
                if n % (x * x) == 0 {
                    // q = n / (x * x) : prime
                    println!("{} {}", x, n / (x * x));
                    found = true;
                    break;
                }

                // q = x
                if n % x == 0 {
                    if let Some(p) = square_root(n / x) {
                        println!("{} {}", p, x);
                        found = true;
                        break;
                    }
                }
            }
        }
        assert!(found);
    }
}
