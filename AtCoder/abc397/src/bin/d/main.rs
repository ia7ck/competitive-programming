use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    // x^3 - y^3 = n
    // (y+a)^3 - y^3 = n
    // 3y^2 + 3ay + a^2 = n/a
    for a in 1_u64.. {
        if a.pow(3) > n {
            break;
        }
        if n % a != 0 {
            continue;
        }
        // y <= 1e9, a <= 1e6
        let f = |y: u64| 3 * y.pow(2) + 3 * a * y + a.pow(2);
        if f(1) > n / a {
            continue;
        }
        let mut ok = 1;
        let mut ng = 1_000_000_000 + 1;
        assert!(f(ok) <= n / a);
        assert!(f(ng) > n / a);
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if f(mid) <= n / a {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if f(ok) == n / a {
            let (x, y) = (ok + a, ok);
            assert_eq!(x.pow(3) - y.pow(3), n);
            println!("{} {}", x, y);
            return;
        }
    }

    println!("-1");
}
