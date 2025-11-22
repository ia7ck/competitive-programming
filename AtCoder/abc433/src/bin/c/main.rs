use proconio::{input, marker::Bytes};

use crate::run_length::RunLength;

fn main() {
    input! {
        s: Bytes,
    };

    let runs = RunLength::new(&s).collect::<Vec<_>>();
    let mut ans = 0_usize;
    for w in runs.windows(2) {
        let (&c1, l1) = w[0];
        let (&c2, l2) = w[1];
        if c1 + 1 == c2 {
            ans += l1.min(l2);
        }
    }
    println!("{}", ans);
}

// 1112222334445556555
// ++++++
//      ++++
//        ++++
//          ++++++
//               ++

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod run_length {
    use std::cmp::Eq;

    pub struct RunLength<'a, T> {
        items: &'a Vec<T>,
        start: usize,
        end: usize,
    }

    impl<'a, T> RunLength<'a, T> {
        pub fn new(items: &'a Vec<T>) -> Self {
            Self {
                items,
                start: 0,
                end: items.len(),
            }
        }
    }

    impl<'a, T> Iterator for RunLength<'a, T>
    where
        T: Eq,
    {
        type Item = (&'a T, usize);

        fn next(&mut self) -> Option<Self::Item> {
            if self.start >= self.end {
                return None;
            }

            let x = &self.items[self.start];
            let mut len = 0;
            while self.start + len < self.end && &self.items[self.start + len] == x {
                len += 1;
            }
            self.start += len;
            Some((x, len))
        }
    }

    impl<'a, T> DoubleEndedIterator for RunLength<'a, T>
    where
        T: Eq,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.start >= self.end {
                return None;
            }

            let x = &self.items[self.end - 1];
            let mut len = 0;
            while self.start < self.end - len && &self.items[self.end - len - 1] == x {
                len += 1;
            }
            self.end -= len;
            Some((x, len))
        }
    }
}
