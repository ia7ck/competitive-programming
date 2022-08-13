use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        e: usize,
        edges: [(Usize1, Usize1); e],
        q: usize,
        xs: [Usize1; q],
    };

    let mut remove_edge = vec![false; e];
    for &x in &xs {
        remove_edge[x] = true;
    }
    let top = n + m;
    let mut uf = UnionFind::new(n + m + 1);
    for (i, &(u, v)) in edges.iter().enumerate() {
        if !remove_edge[i] {
            uf.unite(u, v);
        }
    }
    for j in 0..m {
        uf.unite(top, n + j);
    }
    let mut ans = Vec::new();
    for &x in xs.iter().rev() {
        ans.push(uf.get_size(top) - m - 1);
        let (u, v) = edges[x];
        uf.unite(u, v);
    }

    ans.reverse();
    for ans in ans {
        println!("{}", ans);
    }
}
