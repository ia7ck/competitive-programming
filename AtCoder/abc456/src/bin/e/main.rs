use proconio::{
    input,
    marker::{Chars, Usize1},
};

use crate::detect_cycle::detect_cycle_directed;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            edges: [(Usize1, Usize1); m],
            w: usize,
            s: [Chars; n],
        };

        solve(n, m, edges, w, s);
    }
}

fn solve(n: usize, _m: usize, edges: Vec<(usize, usize)>, w: usize, s: Vec<Vec<char>>) {
    let pack = |v: usize, d: usize| -> usize {
        assert!(v < n);
        assert!(d < w);
        v * w + d
    };

    let edges = {
        let mut edges = edges;
        edges.sort_unstable();
        edges.dedup();
        edges
    };

    let mut new_edges = Vec::new();
    for u in 0..n {
        for d in 0..w {
            if s[u][d] == 'o' && s[u][(d + 1) % w] == 'o' {
                new_edges.push((pack(u, d), pack(u, (d + 1) % w)));
            }
        }
    }
    for (u, v) in edges {
        for d in 0..w {
            if s[u][d] == 'o' && s[v][(d + 1) % w] == 'o' {
                new_edges.push((pack(u, d), pack(v, (d + 1) % w)));
            }
            if s[v][d] == 'o' && s[u][(d + 1) % w] == 'o' {
                new_edges.push((pack(v, d), pack(u, (d + 1) % w)));
            }
        }
    }

    let cycle = detect_cycle_directed(n * w, &new_edges);
    if cycle.is_some() {
        println!("Yes");
    } else {
        println!("No");
    }
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod detect_cycle {
    pub fn detect_cycle_undirected(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
        fn dfs(
            curr: usize,
            prev: usize,
            g: &[Vec<(usize, usize)>],
            seen: &mut Vec<bool>,
            parent: &mut Vec<(usize, usize)>,
        ) -> Option<(usize, usize)> {
            seen[curr] = true;
            for &(nxt, idx) in &g[curr] {
                if nxt == prev {
                    continue;
                }
                if seen[nxt] {
                    return Some((nxt, curr));
                }
                parent[nxt] = (curr, idx);
                if let Some((start, end)) = dfs(nxt, curr, g, seen, parent) {
                    return Some((start, end));
                }
            }
            None
        }

        let mut g = vec![vec![]; n];
        for (idx, &(u, v)) in edges.iter().enumerate() {
            g[u].push((v, idx));
            g[v].push((u, idx));
        }
        let mut seen = vec![false; n];
        let mut parent = vec![(!0, !0); n];

        for v in 0..n {
            if seen[v] {
                continue;
            }
            if let Some((cycle_start, cycle_end)) = dfs(v, !0, &g, &mut seen, &mut parent) {
                let mut cycle = Vec::new();
                {
                    let mut curr = cycle_end;
                    while curr != cycle_start {
                        let (par, idx) = parent[curr];
                        cycle.push(idx);
                        curr = par;
                    }
                }
                for (idx, &(u, v)) in edges.iter().enumerate() {
                    if (u, v) == (cycle_start, cycle_end) || (u, v) == (cycle_end, cycle_start) {
                        cycle.push(idx);
                        return Some(cycle);
                    }
                }
                unreachable!();
            }
        }
        None
    }

    pub fn detect_cycle_directed(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
        fn dfs(
            curr: usize,
            g: &[Vec<(usize, usize)>],
            seen: &mut Vec<bool>,
            on_path: &mut Vec<bool>,
        ) -> Option<(usize, Vec<usize>, bool)> {
            seen[curr] = true;
            on_path[curr] = true;
            for &(nxt, idx) in &g[curr] {
                if on_path[nxt] {
                    assert!(seen[nxt]);
                    return Some((nxt, vec![idx], true));
                }
                if seen[nxt] {
                    continue;
                }
                if let Some((start_node, mut cycle, in_cycle)) = dfs(nxt, g, seen, on_path) {
                    return if in_cycle {
                        cycle.push(idx);
                        if curr == start_node {
                            cycle.reverse();
                            Some((start_node, cycle, false))
                        } else {
                            Some((start_node, cycle, true))
                        }
                    } else {
                        Some((start_node, cycle, false))
                    };
                }
            }
            on_path[curr] = false;
            None
        }

        let mut g = vec![vec![]; n];
        for (idx, &(u, v)) in edges.iter().enumerate() {
            g[u].push((v, idx));
        }
        let mut seen = vec![false; n];
        let mut on_path = vec![false; n];
        for v in 0..n {
            if seen[v] {
                continue;
            }
            if let Some((_, cycle, in_cycle)) = dfs(v, &g, &mut seen, &mut on_path) {
                assert!(!in_cycle);
                return Some(cycle);
            }
        }
        None
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
