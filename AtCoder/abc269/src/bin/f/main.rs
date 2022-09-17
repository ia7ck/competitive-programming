use proconio::input;

fn arithmetic_series(a: u128, n: u128, d: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let last = a + (n - 1) * d;
    (a + last) * n / 2
}

const M: u128 = 998244353;

fn f(_n: u128, m: u128, a: u128, b: u128, c: u128, d: u128) -> u128 {
    if a > b {
        return 0;
    }

    let x = if (a + c) % 2 == 0 {
        (a - 1) * m + c
    } else {
        (a - 1) * m + (c + 1)
    };
    let width = match ((a + c) % 2, (a + d) % 2) {
        (0, 0) => (d - c) / 2 + 1,
        (0, 1) => (d - c) / 2 + 1,
        (1, 0) => (d - c) / 2 + 1,
        (1, 1) => (d - c) / 2,
        _ => unreachable!(),
    };

    let s0 = arithmetic_series(x, width, 2);
    let height = (b - a) / 2 + 1;
    let t = arithmetic_series(0, height, m * 2);
    (s0 * height % M + t * width % M) % M
}

fn solve(n: u128, m: u128, a: u128, b: u128, c: u128, d: u128) {
    let ans = f(n, m, a, b, c, d) + f(n, m, a + 1, b, c, d);
    println!("{}", ans % M);
}

fn main() {
    input! {
        n: u128,
        m: u128,
        q: usize,
    };

    for _ in 0..q {
        input! {
            a: u128,
            b: u128,
            c: u128,
            d: u128,
        };

        solve(n, m, a, b, c, d);
    }
}
