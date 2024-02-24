use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn main() {
    input! {
        n: u64,
        m: u64,
        k: u64,
    };

    let l = lcm(n, m);
    let mut ok = u64::MAX / 2;
    let mut ng = 0;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if (mid / n + mid / m).saturating_sub((mid / l) * 2) >= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
