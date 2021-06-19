use least_prime_factors::least_prime_factors;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let l: usize = rd.get();
    let r: usize = rd.get();

    let lpf = least_prime_factors(r + 1);

    let f = |x: usize, r: usize| {
        let mut factors = Vec::new();
        let mut x = x;
        while x > 1 {
            let p = lpf[x];
            match factors.last() {
                Some(&q) if q != p => {
                    factors.push(p);
                }
                None => {
                    factors.push(p);
                }
                _ => {}
            }
            x /= p;
        }
        let mut res = 0;
        for bits in 1..(1 << factors.len()) {
            let mut product = 1;
            let mut cnt = 0;
            for i in 0..(factors.len()) {
                if bits >> i & 1 == 1 {
                    product *= factors[i];
                    cnt += 1;
                }
            }
            if cnt % 2 == 1 {
                res += r / product;
            } else {
                res -= r / product;
            }
        }
        res
    };

    let mut ans = 0;
    for x in l..=r {
        let a = f(x, r) - f(x, x - 1);
        let b = r / x;
        // eprintln!("{} {} {}", x, a, b);
        if a >= b {
            // ???
            ans += a - b;
        }
    }
    println!("{}", ans * 2);
}
