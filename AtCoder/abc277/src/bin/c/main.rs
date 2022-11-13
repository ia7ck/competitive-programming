use std::collections::{HashMap, HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };

    let mut g = HashMap::<usize, Vec<usize>>::new();
    g.insert(1, Vec::new());
    for (a, b) in ab {
        g.entry(a).or_insert(Vec::new()).push(b);
        g.entry(b).or_insert(Vec::new()).push(a);
    }
    let mut seen = HashSet::new();
    let mut que = VecDeque::new();
    seen.insert(1);
    que.push_back(1);
    while let Some(cur) = que.pop_front() {
        for &next in &g[&cur] {
            if seen.contains(&next) {
                continue;
            }
            seen.insert(next);
            que.push_back(next);
        }
    }
    let ans = seen.iter().max().copied().unwrap();
    println!("{}", ans);
}
