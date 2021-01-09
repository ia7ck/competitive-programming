fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let edges: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let a: usize = rd.get();
            let b: usize = rd.get();
            (a - 1, b - 1)
        })
        .collect();

    let mut uf = UnionFind::new(400000);
    let mut g = vec![vec![]; 400000];
    for (a, b) in edges {
        uf.unite(a, b);
        g[a].push(b);
        g[b].push(a);
    }
    let coms = uf.components();
    let mut ans = 0;
    for nodes in coms {
        let edge_cnt = nodes.iter().map(|&v| g[v].len()).sum::<usize>();
        assert_eq!(edge_cnt % 2, 0);
        ans += std::cmp::min(nodes.len(), edge_cnt / 2);
    }
    println!("{}", ans);
}

pub struct UnionFind {
    n: usize,
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    /// グラフの頂点数 `n` を渡します。
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            n,
            par: (0..n).map(|i| i).collect::<Vec<_>>(),
            size: vec![1; n],
        }
    }
    /// 頂点 `i` の属する連結成分の代表元を返します。
    pub fn find(&mut self, i: usize) -> usize {
        if self.par[i] == i {
            self.par[i]
        } else {
            self.par[i] = self.find(self.par[i]);
            self.par[i]
        }
    }
    /// 頂点 `i` の属する連結成分と頂点 `j` の属する連結成分をつなげます。
    pub fn unite(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return;
        }
        let (i, j) = if self.size[i] >= self.size[j] {
            (i, j)
        } else {
            (j, i)
        };
        self.par[j] = i;
        self.size[i] += self.size[j];
    }
    /// 頂点 `i` の属する連結成分のサイズ (頂点数) を返します。
    ///
    /// # Examples
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// uf.unite(0, 1);
    /// uf.unite(1, 2);
    /// uf.unite(3, 4);
    /// assert_eq!(uf.get_size(0), 3);
    /// assert_eq!(uf.get_size(1), 3);
    /// assert_eq!(uf.get_size(2), 3);
    /// assert_eq!(uf.get_size(3), 2);
    /// assert_eq!(uf.get_size(4), 2);
    /// assert_eq!(uf.get_size(5), 1);
    /// ```
    pub fn get_size(&mut self, i: usize) -> usize {
        let p = self.find(i);
        self.size[p]
    }
    /// 頂点 `i` と頂点 `j` が同じ連結成分に属するかどうかを返します。
    ///  
    /// # Examples
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// assert!(uf.same(0, 0));
    /// assert!(uf.same(3, 3));
    /// assert!(uf.same(5, 5));
    /// uf.unite(0, 1);
    /// uf.unite(1, 2);
    /// uf.unite(3, 4);
    /// assert!(uf.same(0, 1));
    /// assert!(uf.same(1, 2));
    /// assert!(uf.same(0, 2));
    /// assert!(uf.same(3, 4));
    /// ```
    pub fn same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }
    /// 「連結成分に属する頂点のベクタ」のベクタを返します。
    ///
    /// # Examples
    /// ```
    /// use union_find::UnionFind;
    /// let mut uf = UnionFind::new(6);
    /// uf.unite(0, 1);
    /// uf.unite(1, 2);
    /// uf.unite(3, 4);
    /// let components = uf.components();
    /// for (k, c) in components.iter().enumerate() {
    ///     for &i in c {
    ///         for &j in c {
    ///             assert!(uf.same(i, j));
    ///         }
    ///     }
    ///     for d in &components[0..k] {
    ///         for &i in c {
    ///             for &j in d {
    ///                 assert!(!uf.same(i, j));
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn components(&mut self) -> Vec<Vec<usize>> {
        let mut c = vec![vec![]; self.n];
        for i in 0..self.n {
            let p = self.find(i);
            c[p].push(i);
        }
        c.into_iter().filter(|cc| !cc.is_empty()).collect()
    }
    /// 各連結成分の代表元をベクタで返します。`uf.components().iter().map(|c| uf.find(c[0])).collect()` です。
    pub fn leaders(&mut self) -> Vec<usize> {
        self.components().iter().map(|c| self.find(c[0])).collect()
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
        let end = line.find(' ').unwrap_or_else(|| line.len());
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
