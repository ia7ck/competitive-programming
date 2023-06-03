use std::collections::HashSet;

use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
        k: usize,
        xy: [(Usize1, Usize1); k],
        q_: usize,
        pq: [(Usize1, Usize1); q_],
    };

    let mut uf = UnionFind::new(n);
    for (u, v) in edges {
        uf.unite(u, v);
    }
    let mut roots = HashSet::new();
    for (x, y) in xy {
        let rx = uf.find(x);
        let ry = uf.find(y);
        roots.insert((rx, ry));
    }

    for (p, q) in pq {
        let rp = uf.find(p);
        let rq = uf.find(q);
        if roots.contains(&(rp, rq)) || roots.contains(&(rq, rp)) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
