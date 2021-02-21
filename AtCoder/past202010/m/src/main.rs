fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();
    let edges: Vec<(usize, usize)> = (0..(n - 1))
        .map(|_| {
            let a: usize = rd.get();
            let b: usize = rd.get();
            (a - 1, b - 1)
        })
        .collect();
    let queries: Vec<(usize, usize, usize)> = (0..q)
        .map(|_| {
            let u: usize = rd.get();
            let v: usize = rd.get();
            let c: usize = rd.get();
            (u - 1, v - 1, c)
        })
        .collect();
    let mut top: Vec<usize> = (0..n).collect();
    let lca = LowestCommonAncestor::from_edges(n, &edges);
    let par = lca.par.clone();
    let dep = lca.dep.clone();
    let mut color = vec![0; n];
    color[0] = std::usize::MAX;
    for &(u, v, c) in queries.iter().rev() {
        let mut fill = |x: usize, y: usize, c: usize| {
            let mut x = x;
            loop {
                let mut history = vec![x];
                while x != top[x] {
                    x = top[x];
                    history.push(x);
                }
                history.into_iter().for_each(|xx| {
                    top[xx] = x;
                });
                if dep[x] <= dep[y] {
                    break;
                }
                if x != 0 {
                    assert_eq!(color[x], 0);
                    color[x] = c;
                    let p = par[x][0].unwrap();
                    top[x] = top[p];
                    x = p;
                }
            }
        };
        let w = lca.get(u, v);
        fill(u, w, c);
        fill(v, w, c);
    }
    use std::collections::HashMap;
    let mut edges_map = HashMap::new();
    for (&(a, b), i) in edges.iter().zip(0..(n - 1)) {
        edges_map.insert((a, b), i);
        edges_map.insert((b, a), i);
    }
    let mut ans = vec![0; n - 1];
    for i in 1..n {
        let p = par[i][0].unwrap();
        let &idx = edges_map.get(&(i, p)).unwrap();
        ans[idx] = color[i];
    }
    ans.into_iter().for_each(|ans| println!("{}", ans));
}

pub struct LowestCommonAncestor {
    par: Vec<Vec<Option<usize>>>,
    dep: Vec<usize>,
    lg_n: usize,
}

impl LowestCommonAncestor {
    pub fn from_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut g = vec![vec![]; n];
        for &(u, v) in edges {
            g[u].push(v);
            g[v].push(u);
        }
        Self::from_adj_list(&g)
    }
    pub fn from_adj_list(g: &[Vec<usize>]) -> Self {
        fn dfs(
            i: usize,
            p: Option<usize>,
            g: &[Vec<usize>],
            par: &mut [Vec<Option<usize>>],
            dep: &mut [usize],
        ) {
            par[i][0] = p;
            for &j in &g[i] {
                match p {
                    Some(p) if p == j => {}
                    _ => {
                        dep[j] = dep[i] + 1;
                        dfs(j, Some(i), g, par, dep);
                    }
                }
            }
        }
        let log2 = |k| {
            let mut exp: usize = 1;
            let mut m = 1;
            loop {
                if m >= k {
                    break exp;
                }
                exp += 1;
                m *= 2;
            }
        };
        let n = g.len();
        let lg_n = log2(n) + 1;
        let mut par = vec![vec![None; lg_n]; n];
        let mut dep = vec![0; n];
        dfs(0, None, &g, &mut par, &mut dep);
        for j in 1..lg_n {
            for v in 0..n {
                if let Some(p) = par[v][j - 1] {
                    par[v][j] = par[p][j - 1];
                }
            }
        }
        Self { par, dep, lg_n }
    }
    pub fn get(&self, u: usize, v: usize) -> usize {
        let (mut u, mut v) = if self.dep[u] >= self.dep[v] {
            (u, v)
        } else {
            (v, u)
        };
        for i in 0..self.lg_n {
            if (self.dep[u] - self.dep[v]) >> i & 1 == 1 {
                u = self.par[u][i].unwrap();
            }
        }
        if u == v {
            return u;
        }
        for i in (0..self.lg_n).rev() {
            if self.par[u][i] != self.par[v][i] {
                u = self.par[u][i].unwrap();
                v = self.par[v][i].unwrap();
            }
        }
        self.par[u][0].unwrap()
    }
}

pub struct ProconReader<R> {
    r: R,
    line: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            line: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.line.len());
        assert_ne!(&self.line[self.i..=self.i], " ");
        let line = &self.line[self.i..];
        let end = line.find(' ').unwrap_or(line.len());
        let s = &line[..end];
        self.i += end;
        s.parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", self.line))
    }
    fn skip_blanks(&mut self) {
        loop {
            let start = self.line[self.i..].find(|ch| ch != ' ');
            match start {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    self.line.clear();
                    self.i = 0;
                    let num_bytes = self.r.read_line(&mut self.line).unwrap();
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.line = self.line.trim_end_matches(&['\r', '\n'][..]).to_string();
                }
            }
        }
    }
    pub fn get_vec<T>(&mut self, n: usize) -> Vec<T>
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.get()).collect()
    }
}
