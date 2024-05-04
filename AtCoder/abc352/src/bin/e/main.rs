use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut cliques = Vec::new();
    for _ in 0..m {
        input! {
            k: usize,
            c: u64,
            a: [Usize1; k],
        };
        cliques.push((a, c));
    }
    cliques.sort_by_key(|(_, c)| *c);

    let mut uf = UnionFind::new(n);
    let mut accept = 0;
    let mut ans = 0;
    for (a, c) in cliques {
        let u = a[0];
        for &v in &a[1..] {
            if !uf.same(u, v) {
                uf.unite(u, v);
                accept += 1;
                ans += c;
            }
        }
    }
    if accept == n - 1 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
