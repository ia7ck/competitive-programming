use proconio::input;

fn solve(n: u64) {
    if n < 0b111 {
        println!("-1");
        return;
    }

    // max { 2^s ; n - 2^s >= 3 }
    let mut s = 0;
    while (1 << (s + 1)) <= n - 3 {
        s += 1;
    }
    // max { 2^t ; (n - 2^s) - 2^t >= 1 }
    let mut t = 0;
    while (1 << (t + 1)) <= n - (1 << s) - 1 && (t + 1) < s {
        t += 1;
    }
    // max { 2^u ; (n - 2^s - 2^t - 2^u) >= 0 }
    let mut u = 0;
    while (1 << (u + 1)) <= n - (1 << s) - (1 << t) && (u + 1) < t {
        u += 1;
    }
    println!("{}", (1_u64 << s) + (1_u64 << t) + (1_u64 << u));
}

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: u64,
        };
        solve(n);
    }
}
