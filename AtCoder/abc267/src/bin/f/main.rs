use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn calc_dist(start: usize, g: &Vec<Vec<usize>>) -> Vec<u32> {
    let mut dist = vec![0; g.len()];
    let mut que = VecDeque::new();
    que.push_back((start, start));
    while let Some((cur, prev)) = que.pop_front() {
        for &next in &g[cur] {
            if next == prev {
                continue;
            }
            dist[next] = dist[cur] + 1;
            que.push_back((next, cur));
        }
    }
    dist
}

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
        q: usize,
        queries: [(Usize1, u32); q],
    };

    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }

    let dist1 = calc_dist(0, &g);
    let max_dist = dist1.iter().max().copied().unwrap();
    let uuu = dist1.iter().position(|&d| d == max_dist).unwrap();

    let dist2 = calc_dist(uuu, &g);
    let max_dist = dist2.iter().max().copied().unwrap();
    let vvv = dist2.iter().position(|&d| d == max_dist).unwrap();

    eprintln!("uuu = {}, vvv = {}, max_dist = {}", uuu, vvv, max_dist);

    let dist3 = calc_dist(vvv, &g);
    let mut center = 0;
    let mut found = false;
    for i in 0..n {
        let ok = dist2[i] == max_dist / 2 && dist3[i] == max_dist - max_dist / 2;
        if ok {
            center = i;
            found = true;
            break;
        }
    }
    assert!(found);

    eprintln!("center = {}", center);
    let dist = calc_dist(center, &g);
    let mut nodes = vec![vec![]; n + 1];
    for &u in &g[center] {
        let mut que = VecDeque::new();
        nodes[1].push(u);
        que.push_back((u, u));
        while let Some((cur, prev)) = que.pop_front() {
            for &next in &g[cur] {
                if next == center || next == prev {
                    continue;
                }
                nodes[dist[next] as usize].push(next);
                que.push_back((next, cur));
            }
        }
    }

    let lca = LowestCommonAncestor::new(n, &edges, center);
    for (u, k) in queries {
        if dist[u] >= k {
            let mut v = u;
            // ??
            for i in 0..=21 {
                if k >> i & 1 == 1 {
                    v = lca.ancestor(i, v).unwrap();
                }
            }
            println!("{}", v + 1);
        } else {
            if let Some(&v) = nodes[(k - dist[u]) as usize].first() {
                if lca.get(u, v) == center {
                    println!("{}", v + 1);
                    continue;
                }
            }
            if let Some(&v) = nodes[(k - dist[u]) as usize].last() {
                if lca.get(u, v) == center {
                    println!("{}", v + 1);
                    continue;
                }
            }
            println!("-1");
        }
    }
}

pub struct LowestCommonAncestor {
    ancestor: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

const ILLEGAL: usize = std::usize::MAX;

impl LowestCommonAncestor {
    pub fn new(n: usize, edges: &[(usize, usize)], root: usize) -> Self {
        let mut g = vec![vec![]; n];
        for &(u, v) in edges {
            g[u].push(v);
            g[v].push(u);
        }
        let mut depth = vec![0; n];
        let mut parent = vec![ILLEGAL; n];
        let mut stack = vec![(root, ILLEGAL)];
        while let Some((u, p)) = stack.pop() {
            for &v in &g[u] {
                if v != p {
                    depth[v] = depth[u] + 1;
                    parent[v] = u;
                    stack.push((v, u));
                }
            }
        }
        let table_size = {
            let mut size = 1;
            let mut p = 2;
            while p < n {
                size += 1;
                p *= 2;
            }
            size
        };
        let mut ancestor = vec![vec![ILLEGAL; n]; table_size];
        ancestor[0] = parent;
        for i in 1..table_size {
            ancestor[i] = (0..n)
                .map(|v| {
                    if ancestor[i - 1][v] == ILLEGAL {
                        ILLEGAL
                    } else {
                        ancestor[i - 1][ancestor[i - 1][v]]
                    }
                })
                .collect();
        }
        Self { ancestor, depth }
    }

    pub fn get(&self, u: usize, v: usize) -> usize {
        assert!(u < self.depth.len());
        assert!(v < self.depth.len());
        let (mut u, mut v) = if self.depth[u] >= self.depth[v] {
            (u, v)
        } else {
            (v, u)
        };
        assert!(self.depth[u] >= self.depth[v]);
        let depth_diff = self.depth[u] - self.depth[v];
        for i in 0..self.ancestor.len() {
            if depth_diff >> i & 1 == 1 {
                u = self.ancestor[i][u];
            }
        }
        if u == v {
            return u;
        }
        for i in (0..self.ancestor.len()).rev() {
            if self.ancestor[i][u] != self.ancestor[i][v] {
                u = self.ancestor[i][u];
                v = self.ancestor[i][v];
            }
        }
        let lca = self.ancestor[0][u];
        assert_ne!(lca, ILLEGAL);
        lca
    }

    pub fn get_dist(&self, u: usize, v: usize) -> usize {
        let w = self.get(u, v);
        self.depth[u] + self.depth[v] - self.depth[w] * 2
    }

    pub fn depth(&self, u: usize) -> usize {
        self.depth[u]
    }

    pub fn ancestor(&self, i: usize, u: usize) -> Option<usize> {
        if i >= self.ancestor.len() {
            None
        } else {
            let a = self.ancestor[i][u];
            if a == ILLEGAL {
                None
            } else {
                Some(a)
            }
        }
    }
}
