use dijkstra::{dijkstra, Edge};
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let mut edges = Vec::new();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        let a = a - 1;
        let b = b - 1;
        let (a, b) = (a.min(b), a.max(b));
        edges.push((a, b));
        g[a].push(b);
        g[b].push(a);
    }
    let mut parent = vec![!0; n];
    {
        let mut seen = vec![false; n];
        dfs(0, &g, &mut seen, &mut parent);
    }
    let mut tree_edges = Vec::new();
    let mut tree_g = vec![vec![]; n];
    for i in 1..n {
        let p = parent[i];
        tree_edges.push((i.min(p), i.max(p)));
        tree_g[i].push(p);
        tree_g[p].push(i);
    }
    use std::collections::HashSet;
    let tree_edges: HashSet<(usize, usize)> = tree_edges.into_iter().collect();
    let mut nodes = Vec::new();
    for &(s, t) in &edges {
        if !tree_edges.contains(&(s, t)) {
            nodes.push(s);
            nodes.push(t);
        }
    }
    nodes.sort();
    nodes.dedup();
    let g = {
        let mut h = vec![vec![]; n];
        for i in 0..n {
            for &j in &g[i] {
                h[i].push(Edge { to: j, cost: 1 });
                h[j].push(Edge { to: i, cost: 1 });
            }
        }
        h
    };
    let mut dist = Vec::new();
    for i in 0..nodes.len() {
        let start = nodes[i];
        let (d, _) = dijkstra(&g, start);
        let d: Vec<u64> = d.into_iter().map(|d| d.unwrap()).collect();
        dist.push(d);
    }
    let lca = LowestCommonAncestor::new(&tree_g);
    let q: usize = rd.get();
    for _ in 0..q {
        let u: usize = rd.get();
        let v: usize = rd.get();
        let u = u - 1;
        let v = v - 1;
        let mut ans = lca.get_dist(u, v) as u64;
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                // u -> nodes[i] -> nodes[j] -> v
                ans = ans.min(dist[i][u] + dist[i][nodes[j]] + dist[j][v]);
            }
        }
        println!("{}", ans);
    }
}

fn dfs(i: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, parent: &mut Vec<usize>) {
    seen[i] = true;
    for &j in &g[i] {
        if seen[j] {
            continue;
        }
        parent[j] = i;
        dfs(j, g, seen, parent);
    }
}

pub struct LowestCommonAncestor {
    parent: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl LowestCommonAncestor {
    pub fn new(g: &[Vec<usize>]) -> Self {
        let n = g.len();
        let mut depth = vec![0; n];
        let mut parent = vec![!0; n];
        let mut stack = Vec::new();
        stack.push((0, !0));
        while let Some((u, p)) = stack.pop() {
            for &v in &g[u] {
                if v != p {
                    depth[v] = depth[u] + 1;
                    parent[v] = u;
                    stack.push((v, u));
                }
            }
        }
        let n = g.len();
        let table_size = (1..).find(|i| 1_usize << i >= n).unwrap();
        let mut parent = {
            let mut p = vec![vec![!0; n]; table_size];
            p[0] = parent;
            p
        };
        for i in 1..table_size {
            for v in 0..n {
                if parent[i - 1][v] != !0 {
                    parent[i][v] = parent[i - 1][parent[i - 1][v]];
                }
            }
        }
        Self { parent, depth }
    }
    pub fn get(&self, u: usize, v: usize) -> usize {
        let (mut u, mut v) = if self.depth[u] >= self.depth[v] {
            (u, v)
        } else {
            (v, u)
        };
        assert!(self.depth[u] >= self.depth[v]);
        let depth_diff = self.depth[u] - self.depth[v];
        for i in 0..self.parent.len() {
            if depth_diff >> i & 1 == 1 {
                u = self.parent[i][u];
            }
        }
        if u == v {
            return u;
        }
        for i in (0..self.parent.len()).rev() {
            if self.parent[i][u] != self.parent[i][v] {
                u = self.parent[i][u];
                v = self.parent[i][v];
            }
        }
        let lca = self.parent[0][u];
        assert_ne!(lca, !0);
        lca
    }
    pub fn get_dist(&self, u: usize, v: usize) -> usize {
        let w = self.get(u, v);
        self.depth[u] + self.depth[v] - self.depth[w] * 2
    }
}
