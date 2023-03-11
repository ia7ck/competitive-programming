use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [(Usize1, char, Usize1, char); m],
    };

    let vv = |v: usize, c: char| {
        if c == 'R' {
            v
        } else {
            n + v
        }
    };
    let mut uf = UnionFind::new(n * 2);
    for i in 0..n {
        uf.unite(vv(i, 'R'), vv(i, 'B'));
    }
    for &(a, b, c, d) in &abcd {
        uf.unite(vv(a, b), vv(c, d));
    }

    let mut e = vec![0; n * 2];
    for &(a, b, _, _) in &abcd {
        e[uf.find(vv(a, b))] += 1;
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..n * 2 {
        if uf.find(i) == i {
            if uf.get_size(i) == e[i] * 2 {
                ans1 += 1;
            } else {
                ans2 += 1;
            }
        }
    }
    println!("{} {}", ans1, ans2);
}
