use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    // x = 2^a * b^2

    let mut ans = 0;
    for a in 1.. {
        if 2_u64.saturating_pow(a) > n {
            break;
        }
        // 2^a * ok^2 <= n
        let mut ok = 1;
        let mut ng = n;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if 2_u64
                .saturating_pow(a)
                .saturating_mul(mid.saturating_pow(2))
                <= n
            {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ans += (ok + 1) / 2;
    }

    println!("{}", ans);
}
