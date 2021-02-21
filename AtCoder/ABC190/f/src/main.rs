fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);
    let mut ans = 0;
    let mut ft = FenwickTree::new(n, 0i64);
    for i in 0..n {
        ans += ft.sum((a[i] + 1)..n);
        ft.add(a[i], 1);
    }
    for x in a {
        println!("{}", ans);
        let n = n as i64;
        let x = x as i64;
        ans = ans - x + (n - x - 1);
    }
}

pub struct FenwickTree<T> {
    n: usize,
    e: T,
    dat: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Copy,
    T: std::ops::AddAssign,
    T: std::ops::Sub<Output = T>,
    T: std::ops::SubAssign,
{
    pub fn new(n: usize, e: T) -> Self {
        let n = n.next_power_of_two();
        Self {
            n,
            e,
            dat: vec![e; n + 1],
        }
    }
    pub fn add(&mut self, k: usize, x: T) {
        assert!(k < self.n);
        let mut k = (k + 1) as i32;
        while k <= self.n as i32 {
            self.dat[k as usize] += x;
            k += k & (-k);
        }
    }
    fn _sum(&self, r: usize) -> T {
        assert!(r <= self.n);
        let mut result = self.e;
        let mut k = r as i32;
        while k >= 1 {
            result += self.dat[k as usize];
            k -= k & (-k);
        }
        result
    }
    pub fn sum(&self, range: std::ops::Range<usize>) -> T {
        let (l, r) = (range.start, range.end);
        assert!(r <= self.n);
        self._sum(r) - self._sum(l)
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
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", rest));
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
                    self.l.clear();
                    self.i = 0;
                    let num_bytes = self
                        .r
                        .read_line(&mut self.l)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = self
                        .l
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
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
