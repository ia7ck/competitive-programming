use std::{collections::BTreeMap, ops::Range};

use proconio::{input, marker::Usize1};

trait Seq {
    type Output;

    fn len(&self) -> usize;
    fn push_front(&mut self, i: usize);
    fn push_back(&mut self, i: usize);
    fn pop_front(&mut self, i: usize);
    fn pop_back(&mut self, i: usize);
    fn output(&self) -> Self::Output;
}

fn mo<T>(mut seq: T, queries: &[Range<usize>]) -> Vec<T::Output>
where
    T: Seq,
    T::Output: Clone, // :(
{
    let n = seq.len();
    let q = queries.len();
    assert!(queries.iter().all(|range| range.end <= n));
    let b = 1.max(n / (q as f64).sqrt() as usize);
    let mut ord = (0..q).collect::<Vec<_>>();
    ord.sort_by_key(|&i| {
        let Range { start, end } = queries[i];
        let start = start / b;
        // ───>──┐
        //       v
        // ┌──<──┘
        // v
        // └──>──
        if start % 2 == 0 {
            (start, end)
        } else {
            (start, n - end)
        }
    });
    let (mut front, mut back) = (0, 0);
    let mut output = vec![None; q];
    for k in ord {
        let Range { start, end } = queries[k];
        while start < front {
            front -= 1;
            seq.push_front(front);
        }
        while back < end {
            seq.push_back(back);
            back += 1;
        }
        while front < start {
            seq.pop_front(front);
            front += 1;
        }
        while end < back {
            back -= 1;
            seq.pop_back(back);
        }
        output[k] = Some(seq.output());
    }
    output.into_iter().map(Option::unwrap).collect()
}

struct S {
    a: Vec<u64>,
    sum: u64,
    freq: BTreeMap<u64, u64>,
}

impl Seq for S {
    type Output = u64;

    fn len(&self) -> usize {
        self.a.len()
    }

    fn push_front(&mut self, i: usize) {
        let y = self.a[i];
        match (self.freq.range(..y).last(), self.freq.range(y..).next()) {
            (None, Some((&z, &c))) if y < z => {
                assert!(c >= 1);
                self.sum += (z - y) * (z - y);
            }
            (Some((&x, &c)), None) => {
                assert!(c >= 1);
                self.sum += (y - x) * (y - x);
            }
            (Some((&x, &xc)), Some((&z, &zc))) if y < z => {
                assert!(xc >= 1);
                assert!(zc >= 1);
                self.sum -= (z - x) * (z - x);
                self.sum += (z - y) * (z - y);
                self.sum += (y - x) * (y - x);
            }
            _ => {}
        }
        *self.freq.entry(y).or_insert(0) += 1;
    }

    fn push_back(&mut self, i: usize) {
        self.push_front(i);
    }

    fn pop_front(&mut self, i: usize) {
        let y = self.a[i];
        let f = self.freq.get_mut(&y).unwrap();
        *f -= 1;
        if *f == 0 {
            self.freq.remove(&y);
        }
        match (self.freq.range(..y).last(), self.freq.range(y..).next()) {
            (None, Some((&z, &c))) if y < z => {
                assert!(c >= 1);
                self.sum -= (z - y) * (z - y);
            }
            (Some((&x, &c)), None) => {
                assert!(c >= 1);
                self.sum -= (y - x) * (y - x);
            }
            (Some((&x, &xc)), Some((&z, &zc))) if y < z => {
                assert!(xc >= 1);
                assert!(zc >= 1);
                self.sum -= (z - y) * (z - y);
                self.sum -= (y - x) * (y - x);
                self.sum += (z - x) * (z - x);
            }
            _ => {}
        }
    }

    fn pop_back(&mut self, i: usize) {
        self.pop_front(i);
    }

    fn output(&self) -> Self::Output {
        self.sum
    }
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        q: usize,
        queries: [(Usize1, usize); q],
    };

    let seq = S {
        a,
        sum: 0,
        freq: BTreeMap::new(),
    };
    let queries = queries.into_iter().map(|(l, r)| (l..r)).collect::<Vec<_>>();
    let ans = mo(seq, &queries);
    for ans in ans {
        println!("{}", ans);
    }
}
