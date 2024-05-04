use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    };

    let mut pos = vec![0; n];
    for i in 0..n {
        pos[p[i]] = i;
    }

    let mut set = BTreeSet::new();
    for i in 0..k {
        set.insert(pos[i]);
    }
    let mut ans = set.last().unwrap() - set.first().unwrap();
    for i in k..n {
        set.remove(&pos[i - k]);
        set.insert(pos[i]);
        ans = ans.min(set.last().unwrap() - set.first().unwrap());
    }
    println!("{}", ans);
}
