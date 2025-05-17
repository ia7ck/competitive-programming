use memoise::memoise_map;
use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
        };
        solve(n, k);
    }
}

const P: usize = 998244353;

fn solve(n: usize, k: usize) {
    let ans = dp_s(n, k);
    println!("{}", ans);
}

// #{ x ; 0 <= x <= n, popcnt(x) = k }
#[memoise_map(n, k)]
fn dp_c(n: usize, k: usize) -> usize {
    if n == 0 {
        return usize::from(k == 0);
    }
    if k == 0 {
        return 1;
    }

    // 最下位ビット0
    // 2y <= n, popcnt(2y) = popcnt(y) = k

    // 最下位ビット1
    // 2y + 1 <= n, popcnt(2y + 1) = popcnt(2y) + 1 = popcnt(y) = k

    dp_c(n / 2, k) + dp_c((n - 1) / 2, k - 1) % P
}

// sum { x ; 0 <= x <= n, popcnt(x) = k }
#[memoise_map(n, k)]
fn dp_s(n: usize, k: usize) -> usize {
    if n == 0 || k == 0 {
        return 0;
    }

    // 最下位ビット0
    // sum { 2y ; 2y <= n, popcnt(2y) = k }
    // = 2 * sum { y ; y <= n / 2, popcnt(y) = k }

    // 最下位ビット1
    // sum { 2y + 1 <= n, popcnt(2y + 1) = k }
    // = sum { 2y + 1 ; 2y + 1 <= n, popcnt(2y) = popcnt(y) = k - 1 }

    (2 * dp_s(n / 2, k) + 2 * dp_s((n - 1) / 2, k - 1) + dp_c((n - 1) / 2, k - 1)) % P
}
