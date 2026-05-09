use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    };

    let accept = |ans: u64| -> bool {
        let mut cost = 0_u64;
        for (i, &a) in (1..=n).zip(&a) {
            // (ans - a) / i
            let i = i as u64;
            let c = ans.checked_sub(a).map_or(0, |d| (d + (i - 1)) / i);
            cost = cost.saturating_add(c);
        }
        cost <= k
    };

    let mut ok = a.iter().min().copied().unwrap();
    let mut ng = u64::MAX / 3;
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if accept(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
