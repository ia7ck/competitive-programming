fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let strings: Vec<Vec<u64>> = (0..n)
        .map(|_| {
            let s: String = rd.get();
            let mut s = s.chars().map(|c| c as u64).collect::<Vec<_>>();
            s.reverse();
            s
        })
        .collect();
    use rolling_hash::RollingHash;
    let mut hash_vals = vec![];
    for s in &strings {
        let rh = RollingHash::new(s);
        let mut seen = vec![false; 26];
        for i in (0..s.len()).rev() {
            seen[(s[i] - 'a' as u64) as usize] = true;
            let left = rh.get(0..i);
            for j in 0..26 {
                if seen[j] {
                    let right = (j + 'a' as usize) as u64;
                    let hash_val = rh.connect(left, right, 1);
                    hash_vals.push(hash_val);
                }
            }
        }
    }
    hash_vals.sort();
    use binary_search::BinarySearch;
    let mut ans = 0;
    for s in &strings {
        let rh = RollingHash::new(s);
        let hash_val = rh.get(0..s.len());
        let (_, eq, _) = hash_vals.split_by(&hash_val);
        ans += eq.len() - 1;
    }
    println!("{}", ans);
}

mod rolling_hash {
    const MASK30: u64 = (1 << 30) - 1;
    const MASK31: u64 = (1 << 31) - 1;
    const MOD: u64 = (1 << 61) - 1;
    const MASK61: u64 = (1 << 61) - 1;
    const POSITIVIZER: u64 = MOD * 4;
    const BASE: u64 = 1_000_000_000 + 9;
    pub struct RollingHash {
        h: Vec<u64>,
        p: Vec<u64>,
    }
    impl RollingHash {
        pub fn new(s: &[u64]) -> Self {
            let n = s.len();
            let mut h = vec![0; n + 1];
            let mut p = vec![0; n + 1];
            p[0] = 1;
            for i in 0..n {
                h[i + 1] = calc_mod(mul(h[i], BASE) + s[i]);
                p[i + 1] = calc_mod(mul(p[i], BASE));
            }
            Self { h, p }
        }
        pub fn get(&self, range: std::ops::Range<usize>) -> u64 {
            let l = range.start;
            let r = range.end;
            calc_mod(self.h[r] + POSITIVIZER - mul(self.h[l], self.p[r - l]))
        }
        pub fn connect(&self, left: u64, right: u64, right_len: usize) -> u64 {
            calc_mod(mul(left, self.p[right_len]) + right)
        }
    }
    fn mul(a: u64, b: u64) -> u64 {
        let au = a >> 31;
        let ad = a & MASK31;
        let bu = b >> 31;
        let bd = b & MASK31;
        let mid = ad * bu + au * bd;
        let midu = mid >> 30;
        let midd = mid & MASK30;
        au * bu * 2 + midu + (midd << 31) + ad * bd
    }
    fn calc_mod(x: u64) -> u64 {
        let xu = x >> 61;
        let xd = x & MASK61;
        let mut res = xu + xd;
        if res >= MOD {
            res -= MOD;
        }
        res
    }
}

mod binary_search {
    use std::ops::Range;
    pub trait BinarySearch<T> {
        fn lower_bound(&self, x: &T) -> usize;
        fn upper_bound(&self, x: &T) -> usize;
        fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>);
    }

    impl<T: Ord> BinarySearch<T> for [T] {
        // min index self[i] >= x
        // any j (j < i) holds self[j] < x
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
        // any j (j < i) holds self[j] <= x
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
