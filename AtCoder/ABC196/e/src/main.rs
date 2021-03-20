fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let at: Vec<(i64, usize)> = (0..n)
        .map(|_| {
            let a: i64 = rd.get();
            let t: usize = rd.get();
            (a, t)
        })
        .collect();
    let q: usize = rd.get();
    let x: Vec<i64> = rd.get_vec(q);

    let mut high = std::i64::MAX / 3;
    let mut low = std::i64::MIN / 3;
    for &(a, t) in &at {
        match t {
            1 => {
                high += a;
                low += a;
            }
            2 => {
                high = high.max(a);
                low = low.max(a);
            }
            3 => {
                high = high.min(a);
                low = low.min(a);
            }
            _ => unreachable!(),
        }
    }
    let f = |init: i64| {
        let mut y = init;
        for &(a, t) in &at {
            match t {
                1 => {
                    y += a;
                }
                2 => {
                    y = y.max(a);
                }
                3 => {
                    y = y.min(a);
                }
                _ => unreachable!(),
            }
        }
        y
    };
    let left = {
        let (mut ng, mut ok) = (std::i64::MIN / 3, std::i64::MAX / 3);
        while ok - ng > 1 {
            let md = (ok + ng) / 2;
            if f(md) > low {
                ok = md;
            } else {
                ng = md;
            }
        }
        ok
    };
    let right = {
        let (mut ok, mut ng) = (std::i64::MIN / 3, std::i64::MAX / 3);
        while ng - ok > 1 {
            let md = (ok + ng) / 2;
            if f(md) < high {
                ok = md;
            } else {
                ng = md;
            }
        }
        ok
    };
    dbg!(low, high, left, right);
    // assert!(f(left) > low);
    assert_eq!(f(left - 1), low);
    // assert!(f(right) < high);
    assert_eq!(f(right + 1), high);
    let diff = f(left) - left;
    for x in x {
        if x < left {
            println!("{}", low);
        } else if left <= x && x <= right {
            println!("{}", x + diff);
        } else {
            println!("{}", high);
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
