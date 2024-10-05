use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        machines: [(usize, u64, usize, u64); n],
    };

    const INF: u64 = u64::MAX / 2;

    let f = |w: usize| -> bool {
        let mut cost = 0;
        for &(a, p, b, q) in &machines {
            let mut c = INF;
            for i in 0.. {
                if a * i >= w {
                    c = c.min(p * i as u64);
                    break;
                }
                // a * i + b * j >= w
                let j = (w - a * i + (b - 1)) / b;
                c = c.min(p * i as u64 + q * j as u64);
            }
            assert_ne!(c, INF);
            cost += c;
            if cost > x {
                return false;
            }
        }
        true
    };

    let mut ok = 0;
    let mut ng = 1_000_000_000 + 1;
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
