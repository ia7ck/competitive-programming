use std::collections::HashSet;

use proconio::{input, marker::Usize1};

const L: usize = 1_000_000;

fn dfs(i: usize, g: &Vec<Vec<usize>>, seen: &mut HashSet<usize>, count: &mut usize) {
    *count += 1;
    if *count >= L {
        return;
    }

    seen.insert(i);
    for &j in &g[i] {
        if seen.contains(&j) {
            continue;
        }
        dfs(j, g, seen, count);
    }
    seen.remove(&i);
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }

    let mut seen = HashSet::new();
    let mut ans = 0;
    dfs(0, &g, &mut seen, &mut ans);
    println!("{}", ans.min(L));

}
