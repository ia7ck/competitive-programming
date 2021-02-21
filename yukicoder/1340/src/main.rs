fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let t: u64 = rd.get();

    let mut a = vec![vec![0; n]; n];
    for _ in 0..m {
        let u: usize = rd.get();
        let v: usize = rd.get();
        a[v][u] = 1;
    }
    let mut b = vec![vec![vec![0; n]; n]; 64];
    b[0] = a;
    for o in 0..63 {
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    b[o + 1][i][j] |= b[o][i][k] & b[o][k][j];
                }
            }
        }
    }
    let mut y = vec![0; n];
    y[0] = 1;
    for o in 0..64 {
        if t >> o & 1 == 1 {
            let mut nxt = vec![0; n];
            for i in 0..n {
                for j in 0..n {
                    nxt[i] |= b[o][i][j] & y[j];
                }
            }
            y = nxt;
        }
    }
    let ans = y.iter().sum::<usize>();
    println!("{}", ans);
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
