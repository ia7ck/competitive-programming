use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        c: u64,
        xs: [u64; n],
    };

    let mut ans = u64::MAX;
    let l = lcm(a, lcm(b, c));
    ans = ans.min(xs.iter().copied().map(|x| (l - x % l) % l).min().unwrap());
    if n >= 2 {
        for (p, q) in [
            (lcm(a, b), c),
            (c, lcm(a, b)),
            (a, lcm(b, c)),
            (lcm(b, c), a),
            (b, lcm(a, c)),
            (lcm(a, c), b),
        ] {
            let mut xs = xs.clone();
            let mut j = 0;
            for i in 0..n {
                if (p - xs[j] % p) % p > (p - xs[i] % p) % p {
                    j = i;
                }
            }
            let p_cost = (p - xs[j] % p) % p;
            xs.remove(j);
            let q_cost = xs.iter().copied().map(|x| (q - x % q) % q).min().unwrap();
            ans = ans.min(p_cost + q_cost);
        }
    }
    if n >= 3 {
        for (p, q, r) in [
            (a, b, c),
            (a, c, b),
            (b, a, c),
            (b, c, a),
            (c, a, b),
            (c, b, a),
        ] {
            let mut xs = xs.clone();
            let mut j = 0;
            for i in 0..n {
                if (p - xs[j] % p) % p > (p - xs[i] % p) % p {
                    j = i;
                }
            }
            let p_cost = (p - xs[j] % p) % p;
            xs.remove(j);
            let mut j = 0;
            for i in 0..(n - 1) {
                if (q - xs[j] % q) % q > (q - xs[i] % q) % q {
                    j = i;
                }
            }
            let q_cost = (q - xs[j] % q) % q;
            xs.remove(j);
            let r_cost = xs.iter().copied().map(|x| (r - x % r) % r).min().unwrap();
            ans = ans.min(p_cost + q_cost + r_cost);
        }
    }
    println!("{}", ans);
}
