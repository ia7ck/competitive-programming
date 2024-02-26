use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        edges: [(Usize1, Usize1); m],
    };

    let mut uf = UnionFind::new(n);
    let mut directed_edges = Vec::new();
    for (u, v) in edges {
        if a[u] == a[v] {
            uf.unite(u, v);
        } else if a[u] < a[v] {
            directed_edges.push((u, v));
        } else {
            directed_edges.push((v, u));
        }
    }

    directed_edges.sort_unstable_by_key(|&(u, v)| (a[u], a[v]));
    let mut dp = vec![0; n];
    dp[uf.find(0)] = 1;
    for (s, t) in directed_edges {
        let (s, t) = (uf.find(s), uf.find(t));
        if dp[s] >= 1 {
            dp[t] = dp[t].max(dp[s] + 1);
        }
    }

    println!("{}", dp[uf.find(n - 1)]);
}
