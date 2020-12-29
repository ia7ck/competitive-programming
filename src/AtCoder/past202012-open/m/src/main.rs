fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let max: u64 = rd.get();
    let a: Vec<u64> = rd.get_vec(n);

    let mut cul = vec![0; n + 1];
    for i in 0..n {
        cul[i + 1] = cul[i] + a[i];
    }

    let enough = |min: u64| -> bool {
        eprintln!("min={}", min);
        // a[0], ..., a[i-1] をうまくくっつけて
        // 全ての棒について min <= 長さ <= max にできるか
        //
        //  | a[0] | a[1] | ... ... | a[n-1] |
        // ok[0]  ok[1]  ok[2] ... ok[n-1]  ok[n]
        let mut ok = vec![0; n + 2];
        ok[0] += 1;
        ok[1] -= 1;
        for i in 0..n {
            // min <= cul[j] - cul[i] <= max
            let lb = cul.lower_bound(&(min + cul[i]));
            let ub = cul.upper_bound(&(max + cul[i]));
            // eprintln!("i={} lb={} ub={}", i, lb, ub);
            if ok[i] >= 1 && lb <= n {
                ok[lb] += 1;
                ok[ub] -= 1;
            }
            ok[i + 1] += ok[i];
            // eprintln!("{:?}", ok);
        }
        ok[n] >= 1
    };

    let mut ok = *a.iter().min().unwrap();
    let mut ng = max + 1;
    while ng - ok > 1 {
        let md = (ok + ng) / 2;
        if enough(md) {
            ok = md;
        } else {
            ng = md;
        }
    }
    println!("{}", ok);
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
