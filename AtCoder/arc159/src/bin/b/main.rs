use divisors::Divisors;
use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
    };

    // gcd(a - gx, b - gx) = d
    // a - b = (p - q) * d
    let mut ans = 0;
    loop {
        eprintln!("a = {}, b = {}", a, b);
        let g = gcd(a, b);
        let diff = a.max(b) - a.min(b);
        let mut x = std::u64::MAX;
        eprintln!("g = {}, diff = {}", g, diff);
        for d in diff.divisors() {
            eprintln!("d = {}", d);
            if d % g == 0 && d != g {
                x = x.min(a % d / g);
            }
        }
        eprintln!("x = {}", x);
        if x < std::u64::MAX {
            ans += x;
            a -= g * x;
            b -= g * x;
        } else {
            ans += a.min(b) / g;
            break;
        }
    }
    println!("{}", ans);
}
