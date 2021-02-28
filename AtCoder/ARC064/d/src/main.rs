fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let sx: f64 = rd.get();
    let sy: f64 = rd.get();
    let gx: f64 = rd.get();
    let gy: f64 = rd.get();
    let n: usize = rd.get();
    let circles: Vec<(f64, f64, f64)> = (0..n)
        .map(|_| {
            let x: f64 = rd.get();
            let y: f64 = rd.get();
            let r: f64 = rd.get();
            (x, y, r)
        })
        .collect();
    let calc_dist = |(x, y, r): (f64, f64, f64), (xx, yy, rr): (f64, f64, f64)| {
        ((x - xx).hypot(y - yy) - (r + rr)).max(0.0)
    };
    // start: n
    // goal: n+1
    let mut g = vec![vec![std::f64::MAX / 2.0; n + 2]; n + 2];
    g[n][n + 1] = calc_dist((sx, sy, 0.0), (gx, gy, 0.0));
    g[n + 1][n] = calc_dist((sx, sy, 0.0), (gx, gy, 0.0));
    for (i, &(x, y, r)) in circles.iter().enumerate() {
        g[i][n] = calc_dist((x, y, r), (sx, sy, 0.0));
        g[n][i] = calc_dist((x, y, r), (sx, sy, 0.0));
        g[i][n + 1] = calc_dist((x, y, r), (gx, gy, 0.0));
        g[n + 1][i] = calc_dist((x, y, r), (gx, gy, 0.0));
        for (j, &(xx, yy, rr)) in circles.iter().enumerate() {
            if i == j {
                continue;
            }
            g[i][j] = calc_dist((x, y, r), (xx, yy, rr));
            g[j][i] = calc_dist((x, y, r), (xx, yy, rr));
        }
    }
    let mut d = vec![std::f64::MAX / 2.0; n + 2];
    d[n] = 0.0;
    let mut seen = vec![false; n + 2];
    loop {
        let u = (0..(n + 2))
            .filter(|&i| !seen[i])
            .min_by(|&i, &j| d[i].partial_cmp(&d[j]).unwrap());
        if let Some(u) = u {
            seen[u] = true;
            for v in 0..(n + 2) {
                d[v] = d[v].min(d[u] + g[u][v]);
            }
        } else {
            break;
        }
    }
    println!("{}", d[n + 1]);
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
