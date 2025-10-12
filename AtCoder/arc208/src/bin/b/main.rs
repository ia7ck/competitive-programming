use proconio::input;
use rand::{rngs::SmallRng, Rng, SeedableRng};

fn main() {
    if cfg!(debug_assertions) {
        let mut rng = SmallRng::seed_from_u64(123);
        for _ in 0..1000 {
            let n = rng.gen_range(2..10);
            let k = rng.gen_range(1..100);
            solve(n, k);
        }
    }

    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            k: u64,
        };

        solve(n, k);
    }
}

fn solve(n: usize, k: u64) {
    let mut ng = 0;
    let mut ok = k * 2 + 1;
    let mut ans = find(n, k, ok).unwrap();
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if let Some(a) = find(n, k, mid) {
            ok = mid;
            ans = a;
        } else {
            ng = mid;
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn find(n: usize, k0: u64, last: u64) -> Option<Vec<u64>> {
    let mut a = vec![last];
    let mut k = k0;
    for i in 0..(n - 1) {
        let x = a[i] / 2 + 1;
        if a[i] % x <= k {
            k -= a[i] % x;
            a.push(x);
        } else {
            let y = a[i] % x - k;
            assert_eq!(a[i] % (x + y), k);
            k = 0; // k -= a[i] % (x + y)
            a.push(x + y);
        }
    }
    a.reverse();
    let mut s = 0;
    for i in 0..(n - 1) {
        s += a[i + 1] % a[i];
    }
    if s == k0 {
        Some(a)
    } else {
        None
    }
}
