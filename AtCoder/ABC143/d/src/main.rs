fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let ls: Vec<u32> = rd.get_vec(n);

    let mut ls = ls;
    ls.sort();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..i {
            let c = ls[i];
            let b = ls[j];
            // a <= b <= c
            // c - b < a
            let ok = j.saturating_sub(ls.upper_bound(&(c - b)));
            ans += ok;
        }
    }
    println!("{}", ans);
}

use std::ops::Range;
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
    fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>);
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        if self[0] >= *x {
            return 0;
        }
        let mut lf = 0;
        let mut rg = self.len();
        while rg - lf > 1 {
            let md = (rg + lf) / 2;
            if self[md] < *x {
                lf = md;
            } else {
                rg = md;
            }
        }
        rg
    }

    fn upper_bound(&self, x: &T) -> usize {
        if self[0] > *x {
            return 0;
        }
        let mut lf = 0;
        let mut rg = self.len();
        while rg - lf > 1 {
            let md = (rg + lf) / 2;
            if self[md] <= *x {
                lf = md;
            } else {
                rg = md;
            }
        }
        rg
    }

    fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>) {
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
