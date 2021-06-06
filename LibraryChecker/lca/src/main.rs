use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();
    let mut g = vec![vec![]; n];
    for i in 1..n {
        let p: usize = rd.get();
        g[p].push(i);
    }
    let lca = LowestCommonAncestor::new(&g);
    for _ in 0..q {
        let u: usize = rd.get();
        let v: usize = rd.get();
        println!("{}", lca.get(u, v));
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
