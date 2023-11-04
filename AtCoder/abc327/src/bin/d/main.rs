use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    };

    // https://noshi91.hatenablog.com/entry/2018/04/17/183132

    let mut uf = UnionFind::new(n * 2);
    for i in 0..m {
        uf.unite(a[i], b[i] + n);
        uf.unite(a[i] + n, b[i]);
    }
    for i in 0..n {
        if uf.same(i, i + n) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
