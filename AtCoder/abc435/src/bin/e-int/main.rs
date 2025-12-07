use proconio::input;

use crate::disjoint_intervals::{DisjointIntervals, InsertItem};

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, usize); q],
    };

    let mut intervals = DisjointIntervals::new();
    let mut ans = n;
    for (l, r) in queries {
        let new_black = intervals.insert(l..(r + 1), 0, |acc, x| match x {
            InsertItem::New(x) => acc + x.count(),
            InsertItem::Overlap(_) => acc,
        });
        ans -= new_black;
        println!("{}", ans);
    }
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod disjoint_intervals {
    use std::{
        collections::BTreeMap,
        fmt::{self, Debug},
        ops::Range,
    };

    pub struct DisjointIntervals<T> {
        intervals: BTreeMap<T, T>, // [start, end)
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum InsertItem<T> {
        New(Range<T>),
        Overlap(Range<T>),
    }

    impl<T> DisjointIntervals<T>
    where
        T: Ord + Clone + Copy,
    {
        pub fn new() -> Self {
            Self {
                intervals: BTreeMap::new(),
            }
        }

        pub fn len(&self) -> usize {
            self.intervals.len()
        }

        pub fn is_empty(&self) -> bool {
            self.intervals.is_empty()
        }

        pub fn iter(&self) -> impl Iterator<Item = Range<T>> {
            self.intervals.iter().map(|(&start, &end)| start..end)
        }

        pub fn insert<U, F>(&mut self, interval: Range<T>, init: U, f: F) -> U
        where
            F: FnMut(U, InsertItem<T>) -> U,
        {
            assert!(!interval.is_empty());

            let mut acc = init;
            let mut f = f;
            let (insert_start, mut start, insert_end) =
                match self.intervals.range(..=interval.start).last() {
                    Some((&prev_start, &prev_end)) if interval.start == prev_end => {
                        self.intervals.remove(&prev_start);
                        (prev_start, interval.start, interval.end)
                    }
                    Some((&prev_start, &prev_end)) if interval.start < prev_end => {
                        acc = f(acc, InsertItem::Overlap(interval.start..prev_end));
                        self.intervals.remove(&prev_start);
                        (prev_start, prev_end, interval.end.max(prev_end))
                    }
                    _ => (interval.start, interval.start, interval.end),
                };

            while let Some((&next_start, &next_end)) = self.intervals.range(start..=insert_end).next() {
                assert!(start < next_start);
                assert!(next_start <= insert_end);

                acc = f(acc, InsertItem::New(start..next_start));

                if insert_end <= next_end {
                    acc = f(acc, InsertItem::Overlap(next_start..insert_end));
                    self.intervals.remove(&next_start);
                    self.intervals
                        .insert(insert_start, insert_end.max(next_end));
                    return acc;
                }

                acc = f(acc, InsertItem::Overlap(next_start..next_end));
                self.intervals.remove(&next_start);
                start = next_end;
            }

            if start < insert_end {
                acc = f(acc, InsertItem::New(start..insert_end));
            }
            self.intervals.insert(insert_start, insert_end);
            acc
        }
    }

    impl<T> Debug for DisjointIntervals<T>
    where
        T: Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.intervals.iter()).finish()
        }
    }
}
