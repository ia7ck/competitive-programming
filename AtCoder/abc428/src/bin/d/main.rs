use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            c: u64,
            d: u64,
        };

        solve(c, d);
    }
}

fn solve(c: u64, d: u64) {
    let mut ans = 0;
    let n_c = c.to_string().len();
    let n_cd = (c + d).to_string().len();
    for k in n_c..=n_cd {
        // k = (c+x).len()
        // 10^(k-1) - c <= x <= 10^k - 1 - c
        let k = k as u32;
        let lx = 10_u64.pow(k - 1).saturating_sub(c).max(1);
        let rx = (10_u64.pow(k) - 1 - c).min(d);
        if lx > rx {
            continue;
        }
        let upper = c * 10_u64.pow(k);
        ans += count_squares(upper + (c + lx), upper + (c + rx));
    }

    println!("{}", ans);
}

fn count_squares(low: u64, high: u64) -> u64 {
    assert!(low >= 1);
    assert!(high >= 1);
    assert!(low <= high);

    sqrt(high) - sqrt(low - 1)
}

fn sqrt(x: u64) -> u64 {
    let mut ok = 0;
    let mut ng = x + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if mid.saturating_pow(2) <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
