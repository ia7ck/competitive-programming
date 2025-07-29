use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut edges: [(Usize1, Usize1, usize); n - 1],
        queries: [(Usize1, Usize1, usize); q],
    };

    // uf[k] := 重みk以下の辺のみのグラフ
    let mut uf = vec![UnionFind::new(n); 11];
    edges.sort_unstable_by_key(|&(_, _, c)| c);
    let mut ans = 0;
    for (a, b, c) in edges {
        for i in c..10 {
            uf[i].unite(a, b);
        }
        // Kruskal
        if uf[10].unite(a, b) {
            ans += c;
        }
    }

    for (u, v, w) in queries {
        let mut sub = 0;
        for i in w..=10 {
            // true, ..., true, false, ..., false になる
            if uf[i].unite(u, v) {
                sub += 1;
            }
        }
        // 重み w+sub の辺を w で置き換えられる
        ans -= sub;
        println!("{}", ans);
    }

}

// Bundled
#[allow(unused)]
mod union_find {
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        nodes: Vec<NodeKind>,
        groups: usize,
    }

    #[derive(Clone, Copy, Debug)]
    enum NodeKind {
        Root { size: usize },
        Child { parent: usize },
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                nodes: vec![NodeKind::Root { size: 1 }; n],
                groups: n,
            }
        }

        pub fn find(&mut self, i: usize) -> usize {
            assert!(i < self.nodes.len());

            match self.nodes[i] {
                NodeKind::Root { .. } => i,
                NodeKind::Child { parent } => {
                    let root = self.find(parent);
                    if root == parent {
                    } else {
                        self.nodes[i] = NodeKind::Child { parent: root };
                    }
                    root
                }
            }
        }

        pub fn unite(&mut self, i: usize, j: usize) -> bool {
            let i = self.find(i);
            let j = self.find(j);
            if i == j {
                return false;
            }

            match (self.nodes[i], self.nodes[j]) {
                (NodeKind::Root { size: i_size }, NodeKind::Root { size: j_size }) => {
                    let total = i_size + j_size;
                    if i_size >= j_size {
                        self.nodes[j] = NodeKind::Child { parent: i };
                        self.nodes[i] = NodeKind::Root { size: total };
                    } else {
                        self.nodes[i] = NodeKind::Child { parent: j };
                        self.nodes[j] = NodeKind::Root { size: total };
                    }
                }
                _ => unreachable!(),
            }

            self.groups -= 1;
            true
        }

        pub fn size(&mut self, i: usize) -> usize {
            let root = self.find(i);
            match self.nodes[root] {
                NodeKind::Root { size } => size,
                _ => unreachable!(),
            }
        }

        pub fn same(&mut self, i: usize, j: usize) -> bool {
            self.find(i) == self.find(j)
        }

        pub fn count_groups(&self) -> usize {
            self.groups
        }
    }
}
