use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut keep = HashSet::new();
    for (u, v) in edges {
        if u == v {
            continue;
        }
        keep.insert((u.min(v), u.max(v)));
    }

    let ans = m - keep.len();
    println!("{}", ans);
}
