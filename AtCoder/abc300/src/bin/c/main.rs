use proconio::{input, marker::Chars};
use union_find::UnionFind;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let mut uf = UnionFind::new(h * w);
    for i in 0..h {
        for j in 0..w {
            let mut unite = |ni: usize, nj: usize| {
                if c[ni][nj] == '#' {
                    uf.unite(i * w + j, ni * w + nj);
                }
            };
            if c[i][j] == '#' {
                if i >= 1 && j >= 1 {
                    unite(i - 1, j - 1);
                }
                if i >= 1 && j + 1 < w {
                    unite(i - 1, j + 1);
                }
                if i + 1 < h && j >= 1 {
                    unite(i + 1, j - 1);
                }
                if i + 1 < h && j + 1 < w {
                    unite(i + 1, j + 1);
                }
            }
        }
    }
    let mut ans = vec![0; h.min(w) + 1];
    for v in 0..h * w {
        if uf.find(v) == v {
            let s = uf.get_size(v);
            assert_eq!((s - 1) % 4, 0);
            ans[(s - 1) / 4] += 1;
        }
    }
    for i in 1..ans.len() {
        print!("{}", ans[i]);
        if i + 1 < ans.len() {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
