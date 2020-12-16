fn solve(n: usize, _: usize, a: &Vec<u64>, s: &Vec<u64>) {
    let mut a = a.clone();
    a.sort();
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }
    let max = |x, y| std::cmp::max(x, y);
    let min = |x, y| std::cmp::min(x, y);
    let mut seg_max = SegmentTree::new(n, std::u64::MIN, max);
    let mut seg_min = SegmentTree::new(n, std::u64::MAX, min);

    for i in 0..n {
        seg_max.update(i, a[i]);
        seg_min.update(i, a[i]);
    }
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back((0usize, n));
    while let Some((l, r)) = q.pop_front() {
        let sum = cum[r] - cum[l];
        seen.insert(sum);
        let max = seg_max.fold(l..r);
        let min = seg_min.fold(l..r);
        let m = (max + min) / 2;
        let ub = a.upper_bound(&m);
        if ub < r {
            q.push_back((l, ub));
            q.push_back((ub, r));
        }
    }
    for s in s {
        if seen.contains(s) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let q: usize = rd.get();
        let a: Vec<u64> = rd.get_vec(n);
        let s: Vec<u64> = rd.get_vec(q);
        solve(n, q, &a, &s);
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

struct SegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    e: T,
    multiply: F,
}

#[allow(dead_code)]
impl<T, F> SegmentTree<T, F>
where
    T: Clone + Copy,
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
        self._fold(&range, 0, 0..self.n)
    }
    fn _fold(
        &self,
        range: &std::ops::Range<usize>,
        i: usize,
        i_range: std::ops::Range<usize>,
    ) -> T {
        if range.end <= i_range.start || i_range.end <= range.start {
            return self.e;
        }
        if range.start <= i_range.start && i_range.end <= range.end {
            return self.dat[i];
        }
        let m = (i_range.start + i_range.end) / 2;
        (self.multiply)(
            self._fold(range, i * 2 + 1, i_range.start..m),
            self._fold(range, i * 2 + 2, m..i_range.end),
        )
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
