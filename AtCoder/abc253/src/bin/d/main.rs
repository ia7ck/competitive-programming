use arithmetic_series::arithmetic_series;
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
        n: u64,
        a: u64,
        b: u64,
    };

    let lcm = a / gcd(a, b) * b;
    let s_n = arithmetic_series(1, n, 1).unwrap();
    let s_a = if n / a >= 1 {
        arithmetic_series(a, n / a, a).unwrap()
    } else {
        0
    };
    let s_b = if n / b >= 1 {
        arithmetic_series(b, n / b, b).unwrap()
    } else {
        0
    };
    let s_lcm = if n / lcm >= 1 {
        arithmetic_series(lcm, n / lcm, lcm).unwrap()
    } else {
        0
    };
    assert!(s_n + s_lcm >= s_a + s_b);
    println!("{}", s_n + s_lcm - s_a - s_b);
}
