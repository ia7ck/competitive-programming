use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        ud: [(u64, u64); n],
    };

    let (mut u, d): (Vec<_>, Vec<_>) = ud.into_iter().unzip();
    let mut cost_u = 0;
    {
        let mut set = BTreeSet::new();
        for i in 0..n {
            set.insert((u[i], i));
        }
        while let Some((y, i)) = set.pop_first() {
            assert_eq!(u[i], y);
            if i >= 1 && u[i - 1] > u[i] && u[i - 1] - u[i] > x {
                cost_u += u[i - 1] - (u[i] + x);
                set.remove(&(u[i - 1], i - 1));
                u[i - 1] = u[i] + x;
                set.insert((u[i - 1], i - 1));
            }
            if i + 1 < n && u[i] < u[i + 1] && u[i + 1] - u[i] > x {
                cost_u += u[i + 1] - (u[i] + x);
                set.remove(&(u[i + 1], i + 1));
                u[i + 1] = u[i] + x;
                set.insert((u[i + 1], i + 1));
            }
        }
    }
    for i in 0..(n - 1) {
        assert!(u[i].abs_diff(u[i + 1]) <= x);
    }

    let h = u.iter().zip(&d).map(|(u, d)| u + d).collect::<Vec<_>>();
    let cost_h = {
        let min_h = h.iter().min().unwrap();
        h.iter().map(|h| h - min_h).sum::<u64>()
    };

    let ans = cost_u + cost_h;
    println!("{}", ans);
}
