pub struct ProconReader<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
    pub fn get<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&byte| byte == b' ' || byte == b'\n' || byte == b'\r')
            .take_while(|&byte| byte != b' ' && byte != b'\n' && byte != b'\r')
            .collect::<Vec<_>>();
        std::str::from_utf8(&buf)
            .unwrap()
            .parse()
            .ok()
            .expect("Parse Error.")
    }
}

pub struct FenwickTree {
    n: usize,
    dat: Vec<i64>,
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        Self {
            n,
            dat: vec![0; n + 1],
        }
    }
    pub fn add(&mut self, k: usize, x: i64) {
        assert!(1 <= k && k <= self.n);
        let n = self.n as i32;
        let mut k = k as i32;
        while k <= n {
            self.dat[k as usize] += x;
            k += k & (-k);
        }
    }
    pub fn _sum(&self, r: usize) -> i64 {
        assert!(r <= self.n);
        let mut result = 0;
        let mut k = r as i32;
        while k >= 1 {
            result += self.dat[k as usize];
            k -= k & (-k);
        }
        result
    }
    pub fn sum(&self, l: usize, r: usize) -> i64 {
        if l > r {
            return 0;
        }
        assert!(1 <= l && l <= self.n);
        assert!(1 <= r && r <= self.n);
        self._sum(r) - self._sum(l - 1)
    }
}

mod binary_search {
    use std::ops::Range;
    pub trait BinarySearch<T> {
        fn lower_bound(&self, x: &T) -> usize;
        fn upper_bound(&self, x: &T) -> usize;
        fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>);
    }

    // min index self[i] >= x
    // therefore, any j (j < i) holds self[j] < x
    impl<T: Ord> BinarySearch<T> for [T] {
        fn lower_bound(&self, x: &T) -> usize {
            if self[0] >= *x {
                return 0;
            }
            let mut lf = 0;
            let mut rg = self.len();
            // self[lf] < x
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

        // min index self[i] > x
        // therefore, any j (j < i) holds self[j] <= x
        fn upper_bound(&self, x: &T) -> usize {
            if self[0] > *x {
                return 0;
            }
            let mut lf = 0;
            let mut rg = self.len();
            // self[lf] <= x
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
}

use binary_search::BinarySearch;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<i64> = (0..n).map(|_| rd.get()).collect();
    let b: Vec<i64> = (0..n).map(|_| rd.get()).collect();

    if a[0] != b[0] || a[n - 1] != b[n - 1] {
        println!("-1");
        return;
    }

    let a = a.windows(2).map(|w| w[0] ^ w[1]).collect::<Vec<_>>();
    let b = b.windows(2).map(|w| w[0] ^ w[1]).collect::<Vec<_>>();

    let mut c = a.clone();
    c.sort();
    let mut d = b.clone();
    d.sort();
    if c.iter().zip(&d).any(|(cc, dd)| cc != dd) {
        println!("-1");
        return;
    }
    c.dedup();
    let idx = |x: i64| c.lower_bound(&x);
    let m = c.len();
    let mut ques = vec![std::collections::VecDeque::new(); m];
    for (i, &x) in a.iter().enumerate() {
        ques[idx(x)].push_back(i);
    }
    let mut fen = FenwickTree::new(n - 1);
    for i in 1..=(n - 1) {
        fen.add(i, 1);
    }
    let mut ans = 0;
    for x in b {
        let i = ques[idx(x)].pop_front().unwrap();
        ans += fen.sum(1, i);
        fen.add(i + 1, -1);
    }
    println!("{}", ans);
}
