use proconio::{input, marker::Chars};

use crate::fenwick_tree::FenwickTree;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    // A[r] - A[l] > B[r] - B[l]
    // A[r] - B[r] > A[l] - B[l]
    let mut f = FenwickTree::new(n + n + 1, 0);
    f.add(n, 1);
    let mut diff = 0_isize;
    let mut ans = 0_usize;
    for c in s {
        match c {
            'A' => diff += 1,
            'B' => diff -= 1,
            _ => {}
        }
        let i = n.saturating_add_signed(diff);
        ans += f.sum(..i);
        f.add(i, 1);
    }
    println!("{}", ans);
}

// Bundled
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
