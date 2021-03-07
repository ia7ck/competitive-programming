fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    // (value, index)
    let mut freq = SegmentTree::new(n + 1, (std::usize::MAX, std::usize::MAX), |x, y| x.min(y));
    for i in 0..=n {
        freq.update(i, (0, i));
    }
    for i in 0..m {
        let (x, _) = freq.get(a[i]);
        freq.update(a[i], (x + 1, a[i]));
    }
    let mut ans = {
        let (x, mex) = freq.fold(0..(n + 1));
        assert_eq!(x, 0);
        mex
    };
    for i in m..n {
        let (x, _) = freq.get(a[i - m]);
        freq.update(a[i - m], (x - 1, a[i - m]));
        let (y, _) = freq.get(a[i]);
        freq.update(a[i], (y + 1, a[i]));
        let tmp = {
            let (x, mex) = freq.fold(0..(n + 1));
            assert_eq!(x, 0);
            mex
        };
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}

struct SegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    e: T,
    multiply: F,
}

#[allow(dead_code)]
impl<T, F> SegmentTree<T, F>
where
    T: Copy + std::fmt::Debug,
    F: Fn(T, T) -> T,
{
    fn new(n: usize, e: T, multiply: F) -> Self {
        let n = n.next_power_of_two();
        Self {
            n,
            dat: vec![e; n * 2 - 1],
            e,
            multiply,
        }
    }
    fn get(&self, i: usize) -> T {
        self.dat[i + self.n - 1]
    }
    fn update(&mut self, i: usize, x: T) {
        let mut k = i + self.n - 1;
        self.dat[k] = x;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.multiply)(self.dat[k * 2 + 1], self.dat[k * 2 + 2]);
        }
    }
    fn fold(&self, range: std::ops::Range<usize>) -> T {
        self._fold(range, 0, 0..self.n)
    }
    fn _fold(&self, range: std::ops::Range<usize>, i: usize, i_range: std::ops::Range<usize>) -> T {
        let (start, end) = (range.start, range.end);
        let (i_start, i_end) = (i_range.start, i_range.end);
        if end <= i_start || i_end <= start {
            return self.e;
        }
        if start <= i_start && i_end <= end {
            return self.dat[i];
        }
        let m = (i_start + i_end) / 2;
        (self.multiply)(
            self._fold(range.clone(), i * 2 + 1, i_start..m),
            self._fold(range.clone(), i * 2 + 2, m..i_end),
        )
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
