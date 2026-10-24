use std::collections::HashMap;

use proconio::input;

use crate::{
    arg_cmp::{ArgCmp, Quadrant},
    fenwick_tree::FenwickTree,
};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            xy: [(i64, i64); n],
        };

        solve(n, xy);
    }
}

fn solve(n: usize, xy: Vec<(i64, i64)>) {
    // xy.sort_unstable_by_key(|&(x, y)| ArgCmp::new(x, y));

    #[derive(Debug)]
    enum P {
        Orig(i64, i64),
        Sym(i64, i64),
    }
    impl P {
        fn x(&self) -> i64 {
            match self {
                P::Orig(x, _) => *x,
                P::Sym(x, _) => *x,
            }
        }
        fn y(&self) -> i64 {
            match self {
                P::Orig(_, y) => *y,
                P::Sym(_, y) => *y,
            }
        }
    }

    let mut split = Vec::with_capacity(n);

    // First, Second
    let mut xy2 = Vec::new();
    for &(x, y) in &xy {
        xy2.push(P::Orig(x, y));
        let q = ArgCmp::new(x, y).quadrant();
        if matches!(q, Quadrant::First | Quadrant::Second) {
            xy2.push(P::Sym(-x, -y));
        }
    }
    xy2.sort_unstable_by_key(|p| ArgCmp::new(p.x(), p.y()));
    let mut f = FenwickTree::new(xy2.len(), 0_usize);
    let mut sym = HashMap::<(i64, i64), usize>::new();
    for (i, p) in xy2.iter().enumerate() {
        match p {
            P::Orig(_, _) => {
                f.add(i, 1);
            }
            P::Sym(x, y) => {
                sym.insert((*x, *y), i);
            }
        }
    }
    for (i, p) in xy2.iter().enumerate() {
        let P::Orig(x, y) = p else {
            continue;
        };
        let q = ArgCmp::new(*x, *y).quadrant();
        if matches!(q, Quadrant::First | Quadrant::Second) {
            let j = sym[&(-x, -y)];
            // let c = xy2[i..=j]
            //     .iter()
            //     .filter(|p| matches!(p, P::Orig(_, _)))
            //     .count();
            let c = f.sum(i..=j);
            assert!(c >= 1);
            // c, n - c
            // c - 1, n - (c - 1)
            split.push((c, n - c + 1));
        }
    }

    // Third, Fourth
    let mut xy2 = Vec::new();
    for &(x, y) in &xy {
        xy2.push(P::Orig(x, y));
        let q = ArgCmp::new(x, y).quadrant();
        if matches!(q, Quadrant::Third | Quadrant::Fourth) {
            xy2.push(P::Sym(-x, -y));
        }
    }
    xy2.sort_unstable_by_key(|p| ArgCmp::new(p.x(), p.y()));
    let mut f = FenwickTree::new(xy2.len(), 0_usize);
    let mut sym = HashMap::<(i64, i64), usize>::new();
    for (i, p) in xy2.iter().enumerate() {
        match p {
            P::Orig(_, _) => {
                f.add(i, 1);
            }
            P::Sym(x, y) => {
                sym.insert((*x, *y), i);
            }
        }
    }
    for (i, p) in xy2.iter().enumerate() {
        let P::Orig(x, y) = p else {
            continue;
        };
        let q = ArgCmp::new(*x, *y).quadrant();
        if matches!(q, Quadrant::Third | Quadrant::Fourth) {
            let j = sym[&(-x, -y)];
            // let c = xy2[j..=i]
            //     .iter()
            //     .filter(|p| matches!(p, P::Orig(_, _)))
            //     .count();
            let c = f.sum(j..=i);
            assert!(c >= 1);
            split.push((c, n - c + 1));
        }
    }

    let mut ans = 0;
    for (s1, s2) in split {
        ans = ans.max(s1).max(s2);
    }
    println!("{}", ans);
}

// Bundled
// https://github.com/ia7ck/rust-competitive-programming/blob/bd47f3ceb41010a4c658196af9910dc80263545e/libs/arg_cmp/src/lib.rs
#[rustfmt::skip]
#[allow(unused)]
mod arg_cmp {
    use ::std::cmp::Ordering;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct ArgCmp((i64, i64));

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub enum Quadrant {
        First,
        Second,
        Third,
        Fourth,
    }

    impl ArgCmp {
        pub fn new(x: i64, y: i64) -> Self {
            assert_ne!((x, y), (0, 0));
            Self((x, y))
        }

        pub fn x(&self) -> i64 {
            self.0.0
        }

        pub fn y(&self) -> i64 {
            self.0.1
        }

        fn is_lower_half(&self) -> bool {
            self.y() < 0 || (self.y() == 0 && self.x() < 0)
        }

        pub fn quadrant(&self) -> Quadrant {
            match (self.is_lower_half(), self.x().cmp(&0)) {
                (false, Ordering::Greater) => Quadrant::First,
                (false, _) => Quadrant::Second,
                (true, Ordering::Less) => Quadrant::Third,
                (true, _) => Quadrant::Fourth,
            }
        }
    }

    impl Ord for ArgCmp {
        fn cmp(&self, other: &Self) -> Ordering {
            self.is_lower_half()
                .cmp(&other.is_lower_half())
                .then_with(||
                (self.y() * other.x()).cmp(&(self.x() * other.y())))
        }
    }

    impl PartialOrd for ArgCmp {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}

// Bundled
// https://github.com/ia7ck/rust-competitive-programming/blob/bd47f3ceb41010a4c658196af9910dc80263545e/libs/fenwick_tree/src/lib.rs
#[rustfmt::skip]
#[allow(unused)]
mod fenwick_tree {
    use std::ops::{Bound, RangeBounds};

    #[derive(Clone, Debug)]
    pub struct FenwickTree<T> {
        n: usize,
        e: T,
        dat: Vec<T>,
    }

    impl<T> FenwickTree<T>
    where
        T: Copy,
        T: std::ops::AddAssign,
        T: std::ops::SubAssign,
    {
        pub fn new(n: usize, e: T) -> Self {
            Self {
                n,
                e,
                dat: vec![e; n + 1],
            }
        }
        pub fn add(&mut self, k: usize, x: T) {
            assert!(k < self.n);
            let mut k = k + 1;
            while k <= self.n {
                self.dat[k] += x;
                k += 1 << k.trailing_zeros();
            }
        }
        fn _sum(&self, r: usize) -> T {
            assert!(r <= self.n);
            let mut result = self.e;
            let mut k = r;
            while k >= 1 {
                result += self.dat[k];
                k -= 1 << k.trailing_zeros();
            }
            result
        }
        pub fn sum(&self, range: impl RangeBounds<usize>) -> T {
            let start = match range.start_bound() {
                Bound::Included(&start) => start,
                Bound::Excluded(&start) => start + 1,
                Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                Bound::Included(&end) => end + 1,
                Bound::Excluded(&end) => end,
                Bound::Unbounded => self.n,
            };
            assert!(end <= self.n);
            let mut result = self._sum(end);
            result -= self._sum(start);
            result
        }
    }
}
