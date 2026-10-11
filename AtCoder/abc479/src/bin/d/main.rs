use proconio::{input, marker::Usize1};

use crate::segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        l: [u64; n - 1],
        r: [u64; n - 1],
        queries: [(Usize1, Usize1); q],
    };

    const INF: u64 = u64::MAX / 3;

    let mut l = l;
    let mut r = r;
    l.insert(0, INF);
    r.push(INF);

    let mut l_seg = SegmentTree::new(n, INF, |a, b| *a.min(b));
    let mut r_seg = SegmentTree::new(n, INF, |a, b| *a.min(b));
    for i in 0..n {
        l_seg.set(i, l[i]);
        r_seg.set(i, r[i]);
    }

    for (s, t) in queries {
        let ans = if s < t {
            r[s].min(l[s] + r_seg.fold(..s))
        } else {
            l[s].min(r[s] + l_seg.fold((s + 1)..))
        };

        println!("{}", ans);
    }
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod segment_tree {
    use std::fmt;
    use std::ops::{Bound, Index, RangeBounds};

    #[derive(Clone)]
    pub struct SegmentTree<T, F> {
        original_n: usize,
        n: usize,
        dat: Vec<T>,
        e: T,
        multiply: F,
    }

    impl<T, F> SegmentTree<T, F>
    where
        T: Clone,
        F: Fn(&T, &T) -> T,
    {
        pub fn new(n: usize, e: T, multiply: F) -> Self {
            let original_n = n;
            let n = n.next_power_of_two();
            Self {
                original_n,
                n,
                dat: vec![e.clone(); n * 2], // dat[0] is unused
                e,
                multiply,
            }
        }

        pub fn get(&self, i: usize) -> &T {
            assert!(i < self.original_n);
            &self.dat[i + self.n]
        }

        pub fn set(&mut self, i: usize, x: T) {
            self.update(i, |_| x);
        }

        pub fn update<U>(&mut self, i: usize, f: U)
        where
            U: FnOnce(&T) -> T,
        {
            assert!(i < self.original_n);
            let mut k = i + self.n;
            self.dat[k] = f(&self.dat[k]);
            while k > 1 {
                k >>= 1;
                self.dat[k] = (self.multiply)(&self.dat[k << 1], &self.dat[k << 1 | 1]);
            }
        }

        pub fn fold(&self, range: impl RangeBounds<usize>) -> T {
            let start = match range.start_bound() {
                Bound::Included(&start) => start,
                Bound::Excluded(&start) => start + 1,
                Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                Bound::Included(&end) => end + 1,
                Bound::Excluded(&end) => end,
                Bound::Unbounded => self.original_n,
            };
            assert!(start <= end && end <= self.original_n);
            self._fold(start, end)
        }

        pub fn max_right<P>(&self, l: usize, f: P) -> usize
        where
            P: Fn(&T) -> bool,
        {
            assert!(l <= self.original_n);
            assert!(f(&self.e), "f(e) must be true");

            if l == self.original_n {
                return self.original_n;
            }

            let mut l = l + self.n;
            let mut sum = self.e.clone();

            loop {
                while l % 2 == 0 {
                    l >>= 1;
                }

                let new_sum = (self.multiply)(&sum, &self.dat[l]);
                if !f(&new_sum) {
                    while l < self.n {
                        l <<= 1;
                        let new_sum = (self.multiply)(&sum, &self.dat[l]);
                        if f(&new_sum) {
                            sum = new_sum;
                            l += 1;
                        }
                    }
                    return l - self.n;
                }

                sum = new_sum;
                l += 1;

                if (l & (l.wrapping_neg())) == l {
                    break;
                }
            }

            self.original_n
        }

        pub fn min_left<P>(&self, r: usize, f: P) -> usize
        where
            P: Fn(&T) -> bool,
        {
            assert!(r <= self.original_n);
            assert!(f(&self.e), "f(e) must be true");

            if r == 0 {
                return 0;
            }

            let mut r = r + self.n;
            let mut sum = self.e.clone();

            loop {
                r -= 1;
                while r > 1 && r % 2 == 1 {
                    r >>= 1;
                }

                let new_sum = (self.multiply)(&self.dat[r], &sum);
                if !f(&new_sum) {
                    while r < self.n {
                        r = r * 2 + 1;
                        let new_sum = (self.multiply)(&self.dat[r], &sum);
                        if f(&new_sum) {
                            sum = new_sum;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.n;
                }

                sum = new_sum;

                if (r & (r.wrapping_neg())) == r {
                    break;
                }
            }

            0
        }

        fn _fold(&self, mut l: usize, mut r: usize) -> T {
            let mut acc_l = self.e.clone();
            let mut acc_r = self.e.clone();
            l += self.n;
            r += self.n;
            while l < r {
                if l & 1 == 1 {
                    acc_l = (self.multiply)(&acc_l, &self.dat[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    acc_r = (self.multiply)(&self.dat[r], &acc_r);
                }
                l >>= 1;
                r >>= 1;
            }
            (self.multiply)(&acc_l, &acc_r)
        }
    }

    impl<T, F> Index<usize> for SegmentTree<T, F>
    where
        T: Clone,
        F: Fn(&T, &T) -> T,
    {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            self.get(index)
        }
    }

    impl<T, F> fmt::Debug for SegmentTree<T, F>
    where
        T: fmt::Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", &self.dat[self.n..])
        }
    }
}
