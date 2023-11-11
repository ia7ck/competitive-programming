use itertools::Itertools;
use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
        edges: [(Usize1, Usize1, u64); m],
    };

    let mut ans = k;
    for comb in (0..m).combinations(n - 1) {
        let mut uf = UnionFind::new(n);
        let mut ok = true;
        let mut total = 0;
        for j in comb {
            let (u, v, w) = edges[j];
            if uf.same(u, v) {
                ok = false;
                break;
            }
            uf.unite(u, v);
            total += w;
        }
        if ok {
            ans = ans.min(total % k);
        }
    }
    assert_ne!(ans, k);
    println!("{}", ans);
}
