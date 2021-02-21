// ax + by = gcd(a, b) = g を満たす (x, y, g) を返す
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        // ax + 0y = a
        // -> x = 1
        (1, 0, a)
    } else {
        // a = bq + r
        // -> b * (qx + y) + rx = g
        let (q, r) = (a / b, a % b);
        let (s, t, g) = ext_gcd(b, r);
        // s = qx + y
        // t = x
        (t, s - q * t, g)
    }
}

fn solve(n: i64, s: i64, k: i64) {
    let (x, _, g) = ext_gcd(k, n);
    if s % g != 0 {
        println!("{}", -1);
        return;
    }
    let s = s / g;
    let n = n / g;
    let ans = (-s * x).rem_euclid(n);
    println!("{}", ans);
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: i64 = rd.get();
        let s: i64 = rd.get();
        let k: i64 = rd.get();
        solve(n, s, k);
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
