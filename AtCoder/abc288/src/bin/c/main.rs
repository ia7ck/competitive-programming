use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut uf = UnionFind::new(n);
    for (a, b) in edges {
        uf.unite(a, b);
    }

    let mut sub = 0;
    for r in 0..n {
        if uf.find(r) == r {
            sub += uf.get_size(r) - 1;
        }
    }
    assert!(m >= sub);

    println!("{}", m - sub);
}
