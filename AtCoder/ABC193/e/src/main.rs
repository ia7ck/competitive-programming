// g := gcd(a, b)
// ax + by = g を満たす (x, y, g) を返す
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        // ax + 0y = a
        (1, 0, a)
    } else {
        // a := bq + r
        // -> (bq + r) * x + by = g
        // -> b * (qx + y) + rx = g
        let (q, r) = (a / b, a % b);
        let (s, t, g) = ext_gcd(b, r);
        (t, s - q * t, g)
    }
}

fn crt(a1: i64, a2: i64, m1: i64, m2: i64) -> (i64, i64) {
    let (x, _, g) = ext_gcd(m1, m2);
    if a1 % g != a2 % g {
        return (0, 0);
    }
    let m = m1 / g * m2;
    ((a1 + m1 * ((a2 - a1) / g * x % (m2 / g))).rem_euclid(m), m)
}

fn solve(x: i64, y: i64, p: i64, q: i64) {
    let inf = std::i64::MAX;
    let mut ans = inf;
    for yy in 0..y {
        for qq in 0..q {
            let r1 = x + yy;
            let r2 = p + qq;
            // t = r1 (mod 2x+2y)
            // t = r2 (mod p+q)
            let (t, lcm) = crt(r1, r2, x * 2 + y * 2, p + q);
            if lcm != 0 {
                ans = ans.min(t);
            }
        }
    }
    if ans < inf {
        println!("{}", ans);
    } else {
        println!("infinity");
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let x: i64 = rd.get();
        let y: i64 = rd.get();
        let p: i64 = rd.get();
        let q: i64 = rd.get();
        solve(x, y, p, q);
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
