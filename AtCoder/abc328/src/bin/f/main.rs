use proconio::{input, marker::Usize1};

struct UnionFind {
    parent: Vec<usize>,
    diff: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            diff: vec![0; n],
        }
    }

    fn mut_find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let p = self.parent[i];
            let r = self.mut_find(p);
            self.diff[i] += self.diff[p];
            self.parent[i] = r;
            r
        }
    }

    fn unite(&mut self, i: usize, j: usize, w: i64) -> bool {
        let ri = self.mut_find(i);
        let rj = self.mut_find(j);
        if ri == rj {
            return self.diff[j] - self.diff[i] == w;
        }
        self.parent[ri] = rj;
        self.diff[ri] = self.diff[j] - self.diff[i] - w;
        true
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        abd: [(Usize1, Usize1, i64); q],
    };

    let mut uf = UnionFind::new(n);
    let mut ans = Vec::new();
    for (i, (a, b, d)) in abd.into_iter().enumerate() {
        if uf.unite(a, b, d) {
            ans.push(i);
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
