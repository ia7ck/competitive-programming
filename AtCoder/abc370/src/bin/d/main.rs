use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    };

    let mut row = vec![BTreeSet::from_iter(0..w); h];
    let mut col = vec![BTreeSet::from_iter(0..h); w];
    for _ in 0..q {
        input! {
            r: Usize1,
            c: Usize1,
        };

        if row[r].contains(&c) {
            assert!(col[c].contains(&r));
            row[r].remove(&c);
            col[c].remove(&r);
        } else {
            if let Some(x) = row[r].range(..c).last().copied() {
                row[r].remove(&x);
                col[x].remove(&r);
            }
            if let Some(x) = row[r].range((c + 1)..).next().copied() {
                row[r].remove(&x);
                col[x].remove(&r);
            }
            if let Some(x) = col[c].range(..r).last().copied() {
                col[c].remove(&x);
                row[x].remove(&c);
            }
            if let Some(x) = col[c].range((r + 1)..).next().copied() {
                col[c].remove(&x);
                row[x].remove(&c);
            }
        }
    }

    let mut ans = 0;
    for r in 0..h {
        ans += row[r].len();
    }
    println!("{}", ans);
}
