use std::collections::{HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };

    let mut seen = HashSet::new();
    let mut que = VecDeque::new();
    let mut to = vec![vec![]; n + 1];
    for i in 0..n {
        let (a, b) = ab[i];
        if (a, b) == (0, 0) {
            seen.insert(i + 1);
            que.push_back(i + 1);
        } else {
            to[a].push(i + 1);
            to[b].push(i + 1);
        }
    }

    while let Some(v) = que.pop_front() {
        for &x in &to[v] {
            if seen.contains(&x) {
                continue;
            }
            seen.insert(x);
            que.push_back(x);
        }
    }

    println!("{}", seen.len());
}
