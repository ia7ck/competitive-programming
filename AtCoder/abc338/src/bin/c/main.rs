use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [u64; n],
        a: [u64; n],
        b: [u64; n],
    };

    let mut ans = 0;
    for x in 0..=1_000_000 {
        if q.iter().zip(&a).all(|(&q, &a)| q >= a * x) {
            let mut ok = 0;
            let mut ng = 1_000_000 + 1;
            while ng - ok > 1 {
                let mid = (ok + ng) / 2;
                if q.iter()
                    .zip(&a)
                    .zip(&b)
                    .all(|((&q, &a), &b)| q >= a * x + b * mid)
                {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ans = ans.max(x + ok);
        } else {
            break;
        }
    }
    println!("{}", ans);
}
