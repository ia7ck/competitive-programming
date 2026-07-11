use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        rc: [(usize, usize); m],
    };

    let mut rows = vec![HashSet::<usize>::new(); n + 1];
    let mut cols = vec![HashSet::<usize>::new(); n + 1];
    for (r, c) in rc {
        for c in rows[r].drain() {
            cols[c].remove(&r);
        }
        for r in cols[c].drain() {
            rows[r].remove(&c);
        }
        rows[r].insert(c);
        cols[c].insert(r);
    }

    let mut ans = 0;
    for r in rows {
        ans += r.len();
    }

    println!("{}", ans);
}
