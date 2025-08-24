use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut uf = UnionFind::new(n);
    let mut black = vec![false; n];
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                u: Usize1,
                v: Usize1,
            };
            uf.unite(u, v);
        } else if op == 2 {
            input! {
                v: Usize1,
            };
            let root = uf.find(v);
            if black[v] {
                assert!(uf.black[root] >= 1);
                uf.black[root] -= 1;
            } else {
                uf.black[root] += 1;
            }
            black[v] = !black[v];
        } else {
            input! {
                v: Usize1,
            };
            let root = uf.find(v);
            let count = uf.black[root];
            if count > 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

struct UnionFind {
    p: Vec<usize>,
    size: Vec<usize>,
    black: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
            size: vec![0; n],
            black: vec![0; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.p[i] == i {
            return i;
        }

        let root = self.find(self.p[i]);
        self.p[i] = root;
        root
    }

    fn unite(&mut self, i: usize, j: usize) -> bool {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return false;
        }

        let (i, j) = if self.size[i] > self.size[j] {
            (i, j)
        } else {
            (j, i)
        };
        self.p[j] = i;
        self.size[i] += self.size[j];
        self.black[i] += self.black[j];

        true
    }
}
