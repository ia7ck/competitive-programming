use proconio::{input, marker::Usize1};

use crate::union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut uf = UnionFind::new(n);
    let mut remove = Vec::new();
    for i in (0..m).rev() {
        let (u, v) = edges[i];
        if uf.count_groups() > 2 {
            uf.unite(u, v);
        } else {
            assert_eq!(uf.count_groups(), 2);
            if !uf.same(u, v) {
                remove.push(i);
            } else {
                // ???
                uf.unite(u, v);
            }
        }
    }

    const M: usize = 998_244_353;
    let mut ans = 0;
    for i in remove {
        ans += mpow(2, i + 1, M);
        ans %= M;
    }
    println!("{}", ans);
}

fn mpow(a: usize, x: usize, m: usize) -> usize {
    match x {
        0 => 1,
        1 => a % m,
        x if x % 2 == 0 => mpow(a * a % m, x / 2, m),
        _ => a * mpow(a, x - 1, m) % m,
    }
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
