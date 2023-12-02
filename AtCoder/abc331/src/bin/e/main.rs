use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [u64; n],
        b: [u64; m],
        cd: [(Usize1, Usize1); l],
    };

    let mut ds = vec![vec![]; n];
    for (c, d) in cd {
        ds[c].push(d);
    }

    let mut set = BTreeSet::new();
    for i in 0..m {
        set.insert((b[i], i));
    }
    let mut ans = 0;
    for i in 0..n {
        for &d in &ds[i] {
            set.remove(&(b[d], d));
        }
        if let Some(&(b, _)) = set.last() {
            ans = ans.max(a[i] + b);
        }
        for &d in &ds[i] {
            set.insert((b[d], d));
        }
    }
    println!("{}", ans);
}
