use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut uf = UnionFind::new(n);
    for (u, v) in edges {
        uf.unite(u, v);
    }

    let mut total = 0;
    for i in 0..n {
        if uf.find(i) == i {
            let size = uf.get_size(i);
            total += size * (size - 1) / 2;
        }
    }
    assert!(total >= m);
    let ans = total - m;
    println!("{}", ans);
}
