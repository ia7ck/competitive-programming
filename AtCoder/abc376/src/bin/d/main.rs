use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut g = vec![vec![]; n];
    let mut rg = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        rg[v].push(u);
    }

    let dist = dfs(n, 0, &g);
    let rdist = dfs(n, 0, &rg);

    let mut ans = usize::MAX;
    for &(u, v) in &edges {
        if dist[u] < usize::MAX && rdist[v] < usize::MAX {
            ans = ans.min(dist[u] + rdist[v] + 1);
        }
    }
    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn dfs(n: usize, start: usize, g: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut dist = vec![usize::MAX; n];
    let mut que = VecDeque::new();
    dist[start] = 0;
    que.push_back(start);
    while let Some(i) = que.pop_front() {
        for &j in &g[i] {
            if dist[j] == usize::MAX {
                dist[j] = dist[i] + 1;
                que.push_back(j);
            }
        }
    }
    dist
}
