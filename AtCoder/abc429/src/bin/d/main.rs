use proconio::input;

use crate::zarts::SortedSeq;

fn main() {
    input! {
        n: usize,
        m: u64,
        c: usize,
        a: [u64; n],
    };

    let mut a = a;
    a.push(m - 1);
    let za = SortedSeq::new(a.clone());
    let mut cum_sum = vec![0; za.size() * 2 + 1];
    for &a in &a[..n] {
        let i = za.ord(&a);
        cum_sum[i] += 1;
        cum_sum[i + za.size()] += 1;
        if i + za.size() * 2 < cum_sum.len() {
            cum_sum[i + za.size() * 2] += 1;
        }
    }
    // eprintln!("cumsum = {cum_sum:?}");
    for i in 1..cum_sum.len() {
        cum_sum[i] += cum_sum[i - 1];
    }
    let mut ans = 0;
    for i in 0..za.size() {
        let a = za.at(i);
        // cum_sum[r] - cum_sum[i] < c
        let r = cum_sum[(i + 1)..].partition_point(|&s| s < cum_sum[i] + c);
        let r = (i + 1) + r;
        let x = cum_sum[r] - cum_sum[i];
        assert!(x >= c);
        let b = za.at((i + 1) % za.size());
        let w = if a < b { b - a } else { m + b - a };
        ans += x * w as usize;
        // eprintln!("i = {i}, a = {a}, r = {r}, x = {x}, b = {b}, w = {w}");
        // eprintln!("ans = {ans}");
    }
    // eprintln!("cumsum = {cum_sum:?}");

    println!("{}", ans);
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod zarts {
    use std::{
        fmt::{self, Debug},
        ops::Index,
    };

    pub struct SortedSeq<T>(Vec<T>);

    impl<T> FromIterator<T> for SortedSeq<T>
    where
        T: Ord,
    {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            Self::new(iter)
        }
    }

    impl<T> SortedSeq<T>
    where
        T: Ord,
    {
        pub fn new(values: impl IntoIterator<Item = T>) -> Self {
            let mut values = values.into_iter().collect::<Vec<_>>();
            values.sort_unstable();
            values.dedup();
            Self(values)
        }

        pub fn ord(&self, value: &T) -> usize {
            self.0
                .binary_search(value)
                .unwrap_or_else(|_| panic!("not found"))
        }

        pub fn at(&self, index: usize) -> &T {
            &self[index]
        }

        pub fn size(&self) -> usize {
            self.0.len()
        }
    }

    impl<T> Index<usize> for SortedSeq<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }

    impl<T> Debug for SortedSeq<T>
    where
        T: Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
