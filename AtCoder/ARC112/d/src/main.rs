fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let a: Vec<Vec<char>> = (0..h).map(|_| rd.get_chars()).collect();

    let mut a = a;
    a[0][0] = '#';
    a[0][w - 1] = '#';
    a[h - 1][0] = '#';
    a[h - 1][w - 1] = '#';

    let mut uf = UnionFind::new(h + w);
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                uf.unite(i, h + j);
            }
        }
    }
    let mut rs = (0..h).map(|i| uf.find(i)).collect::<Vec<_>>();
    rs.sort();
    rs.dedup();
    let mut cs = (h..(h + w)).map(|i| uf.find(i)).collect::<Vec<_>>();
    cs.sort();
    cs.dedup();
    let ans = (rs.len() - 1).min(cs.len() - 1);
    println!("{}", ans);
}

pub struct UnionFind {
    n: usize,
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            n,
            par: (0..n).map(|i| i).collect::<Vec<_>>(),
            size: vec![1; n],
        }
    }
    pub fn find(&mut self, i: usize) -> usize {
        if self.par[i] == i {
            self.par[i]
        } else {
            self.par[i] = self.find(self.par[i]);
            self.par[i]
        }
    }
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
    pub fn get_size(&mut self, i: usize) -> usize {
        let p = self.find(i);
        self.size[p]
    }
    pub fn same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }
    pub fn components(&mut self) -> Vec<Vec<usize>> {
        let mut c = vec![vec![]; self.n];
        for i in 0..self.n {
            let p = self.find(i);
            c[p].push(i);
        }
        c.into_iter().filter(|cc| !cc.is_empty()).collect()
    }
    pub fn leaders(&mut self) -> Vec<usize> {
        self.components().iter().map(|c| self.find(c[0])).collect()
    }
}

pub struct ProconReader<R> {
    r: R,
    l: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            l: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.l.len()); // remain some character
        assert_ne!(&self.l[self.i..=self.i], " ");
        let rest = &self.l[self.i..];
        let len = rest.find(' ').unwrap_or_else(|| rest.len());
        // parse self.l[self.i..(self.i + len)]
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|e| panic!("{:?}, attempt to read `{}`", e, rest));
        self.i += len;
        val
    }
    fn skip_blanks(&mut self) {
        loop {
            match self.l[self.i..].find(|ch| ch != ' ') {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    let mut buf = String::new();
                    let num_bytes = self
                        .r
                        .read_line(&mut buf)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = buf
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
                    self.i = 0;
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
    pub fn get_chars(&mut self) -> Vec<char> {
        self.get::<String>().chars().collect()
    }
}
