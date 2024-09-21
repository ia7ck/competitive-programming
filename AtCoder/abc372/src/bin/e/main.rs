use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut uf = UnionFind::new(n);
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
        } else {
            input! {
                v: Usize1,
                k: Usize1,
            };
            let r = uf.find(v);
            if uf.nodes[r].len() <= k {
                println!("-1");
            } else {
                println!("{}", uf.nodes[r][k] + 1);
            }
        }
    }
}

struct UnionFind {
    parent: Vec<usize>,
    nodes: Vec<Vec<usize>>, // sorted
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let nodes = (0..n).map(|i| vec![i]).collect();
        Self { parent, nodes }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        let (x, y) = if self.nodes[x].len() < self.nodes[y].len() {
            (x, y)
        } else {
            (y, x)
        };
        self.parent[x] = y;
        let mut i = 0;
        let mut j = 0;
        let mut new_nodes = Vec::new();
        while new_nodes.len() < 10 && i < self.nodes[x].len() && j < self.nodes[y].len() {
            if self.nodes[x][i] > self.nodes[y][j] {
                new_nodes.push(self.nodes[x][i]);
                i += 1;
            } else {
                new_nodes.push(self.nodes[y][j]);
                j += 1;
            }
        }
        while new_nodes.len() < 10 && i < self.nodes[x].len() {
            new_nodes.push(self.nodes[x][i]);
            i += 1;
        }
        while new_nodes.len() < 10 && j < self.nodes[y].len() {
            new_nodes.push(self.nodes[y][j]);
            j += 1;
        }
        self.nodes[y] = new_nodes;
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let px = self.parent[x];
            let root = self.find(px);
            self.parent[x] = root;
            root
        }
    }
}
