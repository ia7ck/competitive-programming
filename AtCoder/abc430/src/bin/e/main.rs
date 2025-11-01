use proconio::{input, marker::Bytes};

use crate::rolling_hash::RollingHash;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            a: Bytes,
            b: Bytes,
        };

        solve(a, b);
    }
}

fn solve(a: Vec<u8>, b: Vec<u8>) {
    let n = a.len();
    let a = {
        let mut new_a = a.clone();
        new_a.extend(a);
        new_a
    };

    let ha = RollingHash::from_iter(a);
    let hb = RollingHash::from_iter(b);
    for i in 0..n {
        if ha.hash(i..(i + n)) == hb.hash(0..n) {
            println!("{}", i);
            return;
        }
    }

    println!("-1");
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod rolling_hash {
    use std::{iter::FromIterator, ops};

    const MASK30: u64 = (1 << 30) - 1;
    const MASK31: u64 = (1 << 31) - 1;
    const MOD: u64 = (1 << 61) - 1;
    const MASK61: u64 = (1 << 61) - 1;
    const POSITIVIZER: u64 = MOD * 4;
    const BASE: u64 = 1_000_000_000 + 9;

    #[derive(Debug, Clone)]
    pub struct RollingHash {
        xs: Vec<u64>,
        hashes: Vec<u64>,
        pows: Vec<u64>,
    }

    impl<T> FromIterator<T> for RollingHash
    where
        T: Into<u64>,
    {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I)
 -> Self {
            let xs = iter.into_iter().map(|x| x.into()).
collect::<Vec<_>>();
            Self::new(&xs)
        }
    }

    impl RollingHash {
        pub fn new(xs: &[u64]) -> Self {
            let n = xs.len();
            let xs = xs.to_vec();
            let mut hashes = vec![0; n + 1];
            let mut pows = vec![1; n + 1];
            for (i, &x) in xs.iter().enumerate() {
                hashes[i + 1] = calc_mod(mul(hashes[i], 
BASE) + x);
                pows[i + 1] = calc_mod(mul(pows[i], BASE
));
            }
            Self { xs, hashes, pows }
        }

        pub fn len(&self) -> usize {
            self.xs.len()
        }

        pub fn is_empty(&self) -> bool {
            self.xs.is_empty()
        }

        pub fn at(&self, i: usize) -> u64 {
            assert!(i < self.len());
            self.xs[i]
        }

        pub fn hash(&self, range: ops::Range<usize>) -> 
u64 {
            let l = range.start;
            let r = range.end;
            assert!(l <= r);
            assert!(r <= self.hashes.len());
            calc_mod(self.hashes[r] + POSITIVIZER - mul(
self.hashes[l], self.pows[r - l]))
        }

        pub fn is_substring(&self, other: &Self) -> bool
 {
            for j in 0..other.len() {
                if j + self.len() > other.len() {
                    break;
                }
                if self.hash(0..self.len()) == other.hash(j..(j + self.len())) {
                    return true;
                }
            }
            false
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
