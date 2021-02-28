fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let xytr: Vec<(f64, f64, f64, f64)> = (0..n)
        .map(|_| {
            let x: f64 = rd.get();
            let y: f64 = rd.get();
            let t: f64 = rd.get();
            let r: f64 = rd.get();
            (x, y, t, r)
        })
        .collect();
    let mut g = vec![vec![0.0; n]; n];
    for (i, &(x, y, t, _)) in xytr.iter().enumerate() {
        for (j, &(xx, yy, _, rr)) in xytr.iter().enumerate() {
            if i == j {
                continue;
            }
            // i -> j
            g[i][j] = (xx - x).hypot(yy - y) / t.min(rr);
        }
    }
    let mut d: Vec<f64> = vec![1e9; n];
    let mut seen = vec![false; n];
    d[0] = 0.0;
    // n times
    loop {
        let u = (0..n)
            .filter(|&i| !seen[i])
            .min_by(|&i, &j| d[i].partial_cmp(&d[j]).unwrap());
        if let Some(u) = u {
            seen[u] = true;
            for v in 0..n {
                d[v] = d[v].min(d[u] + g[u][v]);
            }
        } else {
            break;
        }
    }
    d.sort_by(|d1, d2| d1.partial_cmp(d2).unwrap());
    let mut ans = 0.0f64;
    for i in 1..n {
        ans = ans.max(d[i] + (n - i - 1) as f64);
    }
    println!("{}", ans);
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
