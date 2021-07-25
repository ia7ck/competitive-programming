use divisors::Divisors;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..n {
        let x: u64 = rd.get();
        let y: u64 = rd.get();
        a.push(x);
        b.push(y);
    }

    let c = a[0].divisors();
    let d = b[0].divisors();
    let mut ans = lcm(
        a.iter().copied().fold(0, |acc, x| gcd(acc, x)),
        b.iter().copied().fold(0, |acc, x| gcd(acc, x)),
    );
    for &x in &c {
        for &y in &d {
            let mut ok = true;
            for (&a, &b) in a.iter().zip(b.iter()) {
                if (a % x == 0 && b % y == 0) || (a % y == 0 && b % x == 0) {
                    // ok
                } else {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans = ans.max(lcm(x, y));
            }
        }
    }
    println!("{}", ans);
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
