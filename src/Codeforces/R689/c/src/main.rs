fn solve(n: usize, _: usize, a: &Vec<usize>, rp: &Vec<(usize, f64)>) {
    let a: Vec<usize> = a.iter().map(|&i| i - 1).collect();
    // let rp: Vec<(usize, f64)> = rp.iter().map(|&(r, p)| (r - 1, p)).collect();

    let j = if a[n - 1] != n - 1 {
        n
    } else {
        let mut j = n - 1;
        while let Some(k) = j.checked_sub(1) {
            if a[k] != k {
                break;
            }
            j = k;
        }
        j
    };
    if j == 0 {
        println!("{}", 1);
        return;
    }
    let mut ans = 0.0;
    let mut q = 1.0;
    for &(r, p) in rp {
        if r >= j {
            ans += q * p;
            q *= 1.0 - p;
        }
    }
    println!("{:.18}", ans);
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let m: usize = rd.get();
        let a: Vec<usize> = rd.get_vec(n);
        let rp: Vec<(usize, f64)> = (0..m)
            .map(|_| {
                let r: usize = rd.get();
                let p: f64 = rd.get();
                (r, p)
            })
            .collect();
        solve(n, m, &a, &rp);
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
