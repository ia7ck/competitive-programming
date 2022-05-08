use proconio::input;

fn main() {
    const M: usize = 1_000_000;
    let mut is_prime = vec![true; M + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=M {
        if is_prime[i] {
            for j in ((i * 2)..=M).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    let mut count = vec![0; M + 1];
    let mut prev = vec![0; M + 1];
    for p in 2..=M {
        count[p] = count[p - 1];
        prev[p] = prev[p - 1];
        if is_prime[p] {
            count[p] += 1;
            prev[p] = p;
        }
    }

    input! {
        n: usize,
    };

    let mut ans = 0_usize;
    for q in 2..=n {
        if q * q * q > n {
            break;
        }
        if is_prime[q] {
            let p = prev[(q - 1).min(n / (q * q * q))];
            ans += count[p];
        }
    }
    println!("{}", ans);
}
