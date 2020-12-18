fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();
    let a: Vec<i64> = rd.get_vec(n);
    let mut seg = SegmentTree::new(n, -1, |x, y| std::cmp::max(x, y));
    for i in 0..n {
        seg.update(i, a[i]);
    }
    for _ in 0..q {
        let t: usize = rd.get();
        match t {
            1 => {
                let i: usize = rd.get();
                let v: i64 = rd.get();
                seg.update(i - 1, v);
            }
            2 => {
                let l: usize = rd.get();
                let r: usize = rd.get();
                println!("{}", seg.fold((l - 1)..r));
            }
            3 => {
                let i: usize = rd.get();
                let v: i64 = rd.get();
                // seg.fold(j..m) >= v を満たす最小の j >= i - 1
                let j = seg.position(i - 1, |x| x >= v);
                println!("{}", j.map_or(n + 1, |j| j + 1));
            }
            _ => unreachable!(),
        }
    }
}

use std::ops::Range;

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
    fn fold(&self, range: Range<usize>) -> T {
        self._fold(range, 0, 0..self.n)
    }
    fn _fold(&self, range: Range<usize>, i: usize, i_range: Range<usize>) -> T {
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
    fn position<P>(&self, start: usize, pred: P) -> Option<usize>
    where
        P: Copy,
        P: Fn(T) -> bool,
    {
        assert!(!pred(self.e));
        self._position(start, pred, 0, 0..self.n, self.e).1
    }
    fn _position<P>(
        &self,
        start: usize,
        pred: P,
        i: usize,
        i_range: Range<usize>,
        acc: T,
    ) -> (T, Option<usize>)
    where
        P: Copy,
        P: Fn(T) -> bool,
    {
        let (i_start, i_end) = (i_range.start, i_range.end);
        if i_start + 1 == i_end {
            let acc = (self.multiply)(acc, self.dat[i]);
            return (acc, if pred(acc) { Some(i_start) } else { None });
        }
        let m = (i_start + i_end) / 2;
        if m <= start {
            return self._position(start, pred, i * 2 + 2, m..i_end, acc);
        }
        let res = self._position(start, pred, i * 2 + 1, i_start..m, acc);
        match res {
            (_, Some(_)) => res,
            (v, None) => {
                let acc = (self.multiply)(acc, v);
                // assert_eq!(acc, self.fold(start..m));
                return self._position(start, pred, i * 2 + 2, m..i_end, acc);
            }
        }
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
