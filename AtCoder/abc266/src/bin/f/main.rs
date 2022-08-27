use std::collections::HashSet;

use detect_cycle::detect_cycle_undirected;
use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n],
        q: usize,
        xy: [(Usize1, Usize1); q],
    };

    let cycle_edge_ids = detect_cycle_undirected(n, &edges).unwrap();
    let mut cycle_edges = HashSet::new();
    for j in cycle_edge_ids {
        let (u, v) = edges[j];
        cycle_edges.insert((u, v));
    }
    let mut uf = UnionFind::new(n);
    for &(u, v) in &edges {
        if !cycle_edges.contains(&(u, v)) {
            assert!(!uf.same(u, v));
            uf.unite(u, v);
        }
    }
    for (x, y) in xy {
        if uf.same(x, y) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
