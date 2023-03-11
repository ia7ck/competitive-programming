use proconio::input;

fn f(a: u64, x: u64, m: u64) -> u64 {
    assert!(x >= 1);
    if x == 1 {
        return 1 % m;
    }
    if x % 2 == 0 {
        (1 + a) * f(a * a % m, x / 2, m) % m
    } else {
        assert!(x >= 3);
        (1 + a * (1 + a) % m * f(a * a % m, (x - 1) / 2, m) % m) % m
    }
}

fn main() {
    input! {
        a: u64,
        x: u64,
        m: u64,
    };

    let ans = f(a, x, m);
    println!("{}", ans);
}
