use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(n: u64, d: u64, k: u64) {
    let g = gcd(n, d);
    let x = (k - 1) / (n / g);
    let ans = x + ((k - 1) % (n / g)) * d;
    println!("{}", ans % n);
}

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: u64,
            d: u64,
            k: u64,
        };
        solve(n, d, k);
    }
}
