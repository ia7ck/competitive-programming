struct Tree {
    par: Vec<Vec<Option<usize>>>,
    dep: Vec<usize>,
}

impl Tree {
    pub fn new(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut g = vec![vec![]; n];
        for &(u, v) in edges {
            g[u].push(v);
            g[v].push(u);
        }
        let mut par = vec![None; n];
        let mut dep = vec![0; n];
        Self::dfs(0, None, &g, &mut par, &mut dep);
        let k = ((n as f64).log2()) as usize + 1;
        let mut par = vec![par];
        for i in 0..k {
            par.push(vec![None; n]);
            for j in 0..n {
                if let Some(p) = par[i][j] {
                    par[i + 1][j] = par[i][p];
                }
            }
        }
        Self { par, dep }
    }

    fn dfs(
        i: usize,
        p: Option<usize>,
        g: &Vec<Vec<usize>>,
        par: &mut Vec<Option<usize>>,
        dep: &mut Vec<usize>,
    ) {
        for &j in &g[i] {
            if p.map_or(true, |p| j != p) {
                par[j] = Some(i);
                dep[j] = dep[i] + 1;
                Self::dfs(j, Some(i), g, par, dep);
            }
        }
    }

    pub fn get_lca(&self, u: usize, v: usize) -> usize {
        let (mut u, mut v) = (u, v);
        if self.dep[u] < self.dep[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let k = self.par.len();
        for i in 0..k {
            if ((self.dep[u] - self.dep[v]) >> i & 1) == 1 {
                u = self.par[i][u].unwrap();
            }
        }
        if u == v {
            return u;
        }
        for i in (0..k).rev() {
            if self.par[i][u] != self.par[i][v] {
                u = self.par[i][u].unwrap();
                v = self.par[i][v].unwrap();
            }
        }
        self.par[0][u].unwrap()
    }

    pub fn dist(&self, u: usize, v: usize) -> usize {
        let w = self.get_lca(u, v);
        self.dep[u] + self.dep[v] - self.dep[w] * 2
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();
    let edges: Vec<(usize, usize)> = (1..n)
        .map(|i| {
            let p: usize = rd.get();
            (i, p)
        })
        .collect();
    let tree = Tree::new(n, &edges);
    for _ in 0..q {
        let u: usize = rd.get();
        let v: usize = rd.get();
        println!("{}", tree.get_lca(u, v));
    }
}

pub struct ProconReader<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
    pub fn get<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&byte| byte == b' ' || byte == b'\n' || byte == b'\r')
            .take_while(|&byte| byte != b' ' && byte != b'\n' && byte != b'\r')
            .collect::<Vec<_>>();
        std::str::from_utf8(&buf)
            .unwrap()
            .parse()
            .ok()
            .expect("Parse Error.")
    }
}
