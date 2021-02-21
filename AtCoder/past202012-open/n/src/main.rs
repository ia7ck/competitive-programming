fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();
    let lr: Vec<(u32, u32)> = (0..(n - 1))
        .map(|_| {
            let l: u32 = rd.get();
            let r: u32 = rd.get();
            (l, r)
        })
        .collect();
    let ab: Vec<(u32, usize)> = (0..q)
        .map(|_| {
            let a: u32 = rd.get();
            let b: usize = rd.get();
            (a, b - 1)
        })
        .collect();

    let mut on: Vec<(u32, usize)> = lr
        .iter()
        .zip(0..(n - 1))
        .map(|(&(l, _), i)| (l, i))
        .collect();
    let mut off: Vec<(u32, usize)> = lr
        .iter()
        .zip(0..(n - 1))
        .map(|(&(_, r), i)| (r, i))
        .collect();
    on.sort();
    off.sort();

    let mut abi: Vec<(u32, usize, usize)> =
        ab.iter().zip(0..q).map(|(&(a, b), i)| (a, b, i)).collect();
    abi.sort(); // 年齢

    use std::collections::BTreeSet;

    let mut ans = vec![0; q];
    let (mut i, mut j) = (0, 0);
    let mut ng = BTreeSet::new();
    for i in 0..n {
        ng.insert(i);
    }
    for (a, b, idx) in abi {
        while i < on.len() && on[i].0 <= a {
            let exist = ng.remove(&(on[i].1));
            assert!(exist);
            i += 1;
        }
        while j < off.len() && off[j].0 < a {
            let new = ng.insert(off[j].1);
            assert!(new);
            j += 1;
        }
        // 0 ----- 1 ----- 2 ----- ... ----- b-1 ----- b ----- b+1
        //     0       1       2                  b-1      b
        let left = {
            let less = ng.range(0..b);
            less.last().map_or(0, |&k| k + 1)
        };
        let right = {
            let mut greater = ng.range(b..(n - 1));
            greater.next().map_or(n - 1, |&k| k)
        };
        // println!("ng={:?}", ng);
        // println!("idx={} left={} right={}", idx, left, right);
        ans[idx] = right - left + 1;
    }
    for ans in ans {
        println!("{}", ans);
    }
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
