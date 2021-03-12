fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let k: usize = rd.get();
    let n: usize = rd.get();
    let m: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let mut d: Vec<usize> = a.windows(2).map(|w| w[1] - w[0] - 1).collect();
    d.sort();
    let mut cum = vec![0; n];
    for i in 1..n {
        cum[i] = cum[i - 1] + d[i - 1];
    }

    for _ in 0..m {
        let x: usize = rd.get();
        let y: usize = rd.get();
        if n == 1 {
            let ans = x.min(a[0] - 1) + y.min(k - a[0]) + 1;
            println!("{}", ans);
        } else {
            let z = y + x;
            let c = d.upper_bound(&z);
            let ans = cum[c] + (n - 1 - c) * z + x.min(a[0] - 1) + y.min(k - a[n - 1]) + n;
            println!("{}", ans);
        }
    }
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
    fn split_by(
        &self,
        x: &T,
    ) -> (
        std::ops::Range<usize>,
        std::ops::Range<usize>,
        std::ops::Range<usize>,
    );
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        if self[0].ge(x) {
            return 0;
        }
        let mut lf = 0;
        let mut rg = self.len();
        while rg - lf > 1 {
            let md = (rg + lf) / 2;
            if self[md].lt(x) {
                lf = md;
            } else {
                rg = md;
            }
        }
        rg
    }

    fn upper_bound(&self, x: &T) -> usize {
        if self[0].gt(x) {
            return 0;
        }
        let mut lf = 0;
        let mut rg = self.len();
        while rg - lf > 1 {
            let md = (rg + lf) / 2;
            if self[md].le(x) {
                lf = md;
            } else {
                rg = md;
            }
        }
        rg
    }

    fn split_by(
        &self,
        x: &T,
    ) -> (
        std::ops::Range<usize>,
        std::ops::Range<usize>,
        std::ops::Range<usize>,
    ) {
        let i = self.lower_bound(x);
        let j = self.upper_bound(x);
        (0..i, i..j, j..self.len())
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
