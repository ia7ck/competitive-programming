use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut uf = UnionFind::new(n);
    for &(u, v) in &edges {
        uf.unite(u, v);
    }

    let mut v_count = vec![0; n];
    let mut e_count = vec![0; n];
    for i in 0..n {
        v_count[uf.find(i)] += 1;
    }
    for &(u, _) in &edges {
        e_count[uf.find(u)] += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        if v_count[i] >= 1 {
            assert!(e_count[i] >= (v_count[i] - 1));
            ans += e_count[i] - (v_count[i] - 1);
        }
    }

    println!("{}", ans);
}
