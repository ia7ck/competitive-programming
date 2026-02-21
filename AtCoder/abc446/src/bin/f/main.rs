use std::collections::{BTreeSet, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut graph = vec![vec![]; n];
    for (u, v) in edges {
        graph[u].push(v);
    }

    let mut reach = HashSet::<usize>::new();
    let mut set = BTreeSet::<usize>::new();
    set.insert(0);
    for k in 0..n {
        while let Some(&v) = set.first() {
            if v <= k {
                if reach.insert(v) {
                    for &x in &graph[v] {
                        set.insert(x);
                    }
                }
                set.remove(&v);
            } else {
                break;
            }
        }
        if reach.len() == (k + 1) {
            println!("{}", set.len());
        } else {
            assert!(reach.len() < k + 1);
            println!("-1");
        }
    }
}
