fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let ab: Vec<(usize, usize)> = (0..m)
        .map(|_| {
            let a: usize = rd.get();
            let b: usize = rd.get();
            (a - 1, b - 1)
        })
        .collect();
    let mut deg = vec![0; n];
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        deg[a] += 1;
        deg[b] += 1;
        g[a].push(b);
        g[b].push(a);
    }
    let sqrt_m = (m as f64).sqrt() as usize;
    let mut friends = vec![vec![]; n];
    for i in 0..n {
        friends[i] = g[i].iter().filter(|&&j| deg[j] > sqrt_m).collect();
    }
    let mut recv = vec![0; n];
    let mut sent = vec![0; n];
    let mut prev = vec![0; n];
    let q: usize = rd.get();
    for _ in 0..q {
        let t: usize = rd.get();
        let x: usize = rd.get();
        let x = x - 1;
        match t {
            1 => {
                if deg[x] <= sqrt_m {
                    for &y in &g[x] {
                        recv[y] += 1;
                    }
                } else {
                    sent[x] += 1;
                }
            }
            2 => {
                let tot = recv[x] + friends[x].iter().map(|&&y| sent[y]).sum::<u64>();
                println!("{}", tot - prev[x]);
                prev[x] = tot;
            }
            _ => unreachable!(),
        }
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
