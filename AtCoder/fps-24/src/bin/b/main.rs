use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    let mut ans = 0;
    for a in [0, 1] {
        for b in [0, 1, 2] {
            if let Some(n) = n.checked_sub(a + b) {
                ans += solve(n);
                ans %= 998244353;
            }
        }
    }
    println!("{}", ans);
}

// 2i + 3j = n
fn solve(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    // n = 6k + 1
    if (n - 1) % 6 == 0 {
        n / 6
    } else {
        n / 6 + 1
    }
}

#[allow(unused)]
fn naive(n: u64) -> u64 {
    let mut res = 0;
    for c in (0..=n).step_by(2) {
        if (n - c) % 3 == 0 {
            res += 1;
        }
    }
    res
}
