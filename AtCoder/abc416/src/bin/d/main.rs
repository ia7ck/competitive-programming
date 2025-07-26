use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            m: u64,
            a: [u64; n],
            b: [u64; n],
        };

        solve(n, m, a, b);
    }
}

fn solve(_n: usize, m: u64, mut a: Vec<u64>, b: Vec<u64>) {
    a.sort_unstable();
    a.reverse();

    let mut b = b.into_iter().enumerate().map(|(i, b)| (b, i)).collect::<BTreeSet<_>>();
    let mut pairs = Vec::new();
    for a in a {
        match b.range((m - a, 0)..).next().copied() {
            Some((x, i)) => {
                pairs.push((a, x));
                b.remove(&(x, i));
            },
            None => {
                let (x, _) = b.pop_last().unwrap();
                pairs.push((a, x));
            },
        }
    }

    let mut ans = 0;
    for (a, b) in pairs {
        ans += (a + b) % m;
    }
    println!("{}", ans);
}

// m = 201
// (187+63)%m + (176+73)%m + (154+85)%m + (150+96)%m + (144+109)%m + (136+156)%m + (111+171)%m + 38+46+110 + 1+7+13
