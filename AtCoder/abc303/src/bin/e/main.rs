use std::{
    assert_eq,
    collections::{HashSet, VecDeque},
};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut g = vec![vec![]; n];
    let mut deg = vec![0; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
        deg[u] += 1;
        deg[v] += 1;
    }
    
    let mut seen = HashSet::new();
    let mut que = VecDeque::new();
    for v in 0..n {
        if deg[v] == 1 {
            seen.insert(v);
            que.push_back(v);
        }
    }
    let mut ans = Vec::new();
    while let Some(u) = que.pop_front() {
        if deg[u] == 0 {
            continue;
        }
        assert_eq!(deg[u], 1, "u = {}", u);
        deg[u] -= 1;
        let vs = g[u]
            .iter()
            .copied()
            .filter(|&v| seen.contains(&v) == false && deg[v] >= 1)
            .collect::<Vec<_>>();
        assert_eq!(vs.len(), 1);
        let v = vs[0];
        assert!(deg[v] >= 2);
        ans.push((v, deg[v]));
        deg[v] = 0;
        for &x in &g[v] {
            if x == u {
                continue;
            }
            if deg[x] == 0 {
                continue;
            }
            deg[x] = 0;
            for &y in &g[x] {
                if y == v {
                    continue;
                }
                if deg[y] == 0 {
                    continue;
                }
                assert!(deg[y] >= 2);
                deg[y] -= 1;
                if deg[y] == 1 {
                    seen.insert(y);
                    que.push_back(y);
                }
            }
        }
    }

    ans.sort_by_key(|&(_, d)| d);
    for i in 0..ans.len() {
        let (_, d) = ans[i];
        print!("{}", d);
        if i + 1 < ans.len() {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
