fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let xy: String = rd.get();
    let mut xy = xy.split('/');
    let x: i64 = xy.next().unwrap().parse().unwrap();
    let y: i64 = xy.next().unwrap().parse().unwrap();
    let g = gcd(x, y);
    let x = x / g;
    let y = y / g;

    // (n * (n+1)/2 - m)/n = x/y
    // n = (x/y + m/n) * 2 - 1, 0 < m/n <= 1
    let mut any = false;
    for &k in &[-1, 0, 1] {
        let n = 2 * x / y + k;
        if n >= 1 {
            let numer = n * (n + 1) * y - n * x * 2;
            let denom = y * 2;
            if numer % denom == 0 {
                let m = numer / denom;
                if 1 <= m && m <= n {
                    assert_eq!((n * (n + 1) / 2 - m) * y, x * n);
                    any = true;
                    println!("{} {}", n, m);
                }
            }
        }
    }
    if !any {
        println!("Impossible");
    }
}

// a >= b
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
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
