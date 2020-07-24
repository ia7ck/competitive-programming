extern crate proconio;
use proconio::input;

mod binary_search {
    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::ops::Range;
    pub trait BinarySearch<T> {
        fn lower_bound(&self, x: &T) -> usize;
        fn upper_bound(&self, x: &T) -> usize;
        fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>);
    }

    // min index self[i] >= x
    // that is, any j (j < i) holds self[j] < x
    impl<T: Ord> BinarySearch<T> for [T] {
        fn lower_bound(&self, x: &T) -> usize {
            match self[0].cmp(x) {
                Greater | Equal => {
                    return 0;
                }
                _ => (),
            }
            let mut lf = 0;
            let mut rg = self.len();
            // self[lf] < x
            while rg - lf > 1 {
                let md = (rg + lf) / 2;
                match self[md].cmp(x) {
                    Less => {
                        lf = md;
                    }
                    Greater | Equal => {
                        rg = md;
                    }
                }
            }
            rg
        }

        // min index self[i] > x
        // that is, any j (j < i) holds self[j] <= x
        fn upper_bound(&self, x: &T) -> usize {
            match self[0].cmp(x) {
                Greater => {
                    return 0;
                }
                _ => (),
            }
            let mut lf = 0;
            let mut rg = self.len();
            // self[lf] <= x
            while rg - lf > 1 {
                let md = (rg + lf) / 2;
                match self[md].cmp(x) {
                    Less | Equal => {
                        lf = md;
                    }
                    Greater => {
                        rg = md;
                    }
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
}

use binary_search::BinarySearch;

fn main() {
    input! {
      n: usize,
      mut a: [i32; n],
    }
    a.sort();
    let mut b = vec![0; n];
    for (i, x) in a.iter().enumerate() {
        let (l, eq, _) = a.split_by(&(x / 2));
        b[i] = l.len() + eq.len();
    }
    let mut ans = 0;
    let mo = 1_000_000_000 + 7;
    for x in &a {
        let (l, eq, _) = a.split_by(&(x / 2));
        let mut s = 0;
        for j in l {
            s += b[j];
        }
        for j in eq {
            s += b[j];
        }
        let (_, eq, r) = a.split_by(&(x * 2));
        let t = eq.len() + r.len();
        ans += s * t;
        ans %= mo;
    }
    println!("{}", ans);
}
