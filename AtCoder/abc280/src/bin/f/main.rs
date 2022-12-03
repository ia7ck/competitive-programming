use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        edges: [(Usize1, Usize1, i64); m],
    };

    let mut uf = UnionFind::new(n);
    for &(a, b, _) in &edges {
        uf.unite(a, b);
    }

    let mut g = vec![vec![]; n];
    for &(a, b, c) in &edges {
        g[a].push((b, c));
        g[b].push((a, -c));
    }

    let mut cost = vec![None; n];
    let mut inf = HashSet::new();
    for s in 0..n {
        if uf.find(s) != s {
            continue;
        }
        let mut que = VecDeque::new();
        cost[s] = Some(0);
        que.push_back(s);
        while let Some(u) = que.pop_front() {
            for &(v, c) in &g[u] {
                let new_cost = cost[u].unwrap() + c;
                if let Some(cc) = cost[v] {
                    if cc != new_cost {
                        inf.insert(s);
                    }
                } else {
                    cost[v] = Some(new_cost);
                    que.push_back(v);
                }
            }
        }
    }

    for _ in 0..q {
        input! {
            x: Usize1,
            y: Usize1,
        };
        if !uf.same(x, y) {
            println!("nan");
            continue;
        }
        let root = uf.find(x);
        if inf.contains(&root) {
            println!("inf");
            continue;
        }
        let ans = cost[y].unwrap() - cost[x].unwrap();
        println!("{}", ans);
    }
}
