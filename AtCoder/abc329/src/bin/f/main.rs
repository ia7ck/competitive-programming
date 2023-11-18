use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [Usize1; n],
        queries: [(Usize1, Usize1); q],
    };

    let mut sets = vec![HashSet::new(); n];
    for i in 0..n {
        sets[i].insert(c[i]);
    }
    let mut index = Vec::from_iter(0..n);
    for (a, b) in queries {
        if sets[index[a]].len() > sets[index[b]].len() {
            index.swap(a, b);
        }
        let moves = sets[index[a]].drain().collect::<Vec<_>>();
        sets[index[b]].extend(moves);
        println!("{}", sets[index[b]].len());
    }
}
