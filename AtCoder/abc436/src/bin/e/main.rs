use proconio::{input, marker::Usize1};

use crate::union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };

    let mut uf = UnionFind::new(n);
    for i in 0..n {
        uf.unite(i, p[i]);
    }
    let mut cmp = vec![vec![]; n];
    for i in 0..n {
        cmp[uf.find(i)].push(i);
    }

    let mut ans = 0;
    for i in 0..n {
        if cmp[i].len() >= 2 {
            ans += cmp[i].len() * (cmp[i].len() - 1) / 2;
        }
    }
    println!("{}", ans);
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod union_find {
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        nodes: Vec<NodeKind>,
        groups: usize,
    }

    #[derive(Clone, Debug)]
    pub struct UniteResult {
        pub new_root: usize,
        pub child: usize,
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

        pub fn unite(&mut self, i: usize, j: usize) -> Option<UniteResult> {
            let i = self.find(i);
            let j = self.find(j);
            if i == j {
                return None;
            }

            match (self.nodes[i], self.nodes[j]) {
                (NodeKind::Root { size: i_size }, NodeKind::Root { size: j_size }) => {
                    self.groups -= 1;
                    let total = i_size + j_size;
                    if i_size >= j_size {
                        self.nodes[j] = NodeKind::Child { parent: i };
                        self.nodes[i] = NodeKind::Root { size: total };
                        Some(UniteResult {
                            new_root: i,
                            child: j,
                        })
                    } else {
                        self.nodes[i] = NodeKind::Child { parent: j };
                        self.nodes[j] = NodeKind::Root { size: total };
                        Some(UniteResult {
                            new_root: j,
                            child: i,
                        })
                    }
                }
                _ => unreachable!(),
            }
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
