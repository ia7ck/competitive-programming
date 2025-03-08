use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, u64); m],
    };

    let mut g = vec![vec![]; n];
    for &(u, v, w) in &edges {
        g[u].push((v, w));
        g[v].push((u, w));
    }

    let mut path = HashSet::from([n - 1]);
    let ans = dfs(n - 1, 0, &g, &mut path);
    assert_ne!(ans, u64::MAX);
    println!("{}", ans);
}

fn dfs(i: usize, acc: u64, g: &Vec<Vec<(usize, u64)>>, path: &mut HashSet<usize>) -> u64 {
    if i == 0 {
        return acc;
    }

    let mut res = u64::MAX;
    for &(j, w) in &g[i] {
        if path.contains(&j) {
            continue;
        }
        path.insert(j);
        res = res.min(dfs(j, acc ^ w, g, path));
        path.remove(&j);
    }
    res
}
