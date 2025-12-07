use std::collections::HashMap;

use proconio::{input, marker::Usize1};

use crate::disjoint_intervals::DisjointIntervals;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u32; n],
        queries: [(Usize1, u32); q],
    };

    let mut counter = HashMap::new();
    let mut intervals = DisjointIntervals::new();
    for &x in &a {
        *counter.entry(x).or_insert(0) += 1;
        intervals.insert(x..(x + 1), (), |_, _| ());
    }

    for (i, x) in queries {
        let c = counter.entry(a[i]).or_insert_with(|| unreachable!());
        *c -= 1;
        if *c == 0 {
            counter.remove(&a[i]);
            intervals.remove(a[i]..(a[i] + 1), (), |_, _| ());
        }

        a[i] = x;
        *counter.entry(a[i]).or_insert(0) += 1;
        intervals.insert(a[i]..(a[i] + 1), (), |_, _| ());

        let g = intervals.ge(0).unwrap();
        if g.start > 0 {
            println!("0");
        } else {
            println!("{}", g.end);
        }
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
        intervals: BTreeMap<T, T>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum InsertItem<T> {
        New(Range<T>),
        Overlap(Range<T>),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum RemoveItem<T> {
        Remove(Range<T>),
        Absent(Range<T>),
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
                    Some((&prev_start, &prev_end)) if interval.start <= prev_end => {
                        if interval.start < prev_end {
                            acc = f(acc, InsertItem::Overlap(interval.start..prev_end));
                        }
                        self.intervals.remove(&prev_start);
                        (
                            prev_start,
                            prev_end.max(interval.start),
                            interval.end.max(prev_end),
                        )
                    }
                    _ => (interval.start, interval.start, interval.end),
                };

            while let Some((&next_start, &next_end)) = self.intervals.range(start..=insert_end).next() {
                assert!(start < next_start);
                assert!(next_start <= insert_end);

                acc = f(acc, InsertItem::New(start..next_start));

                self.intervals.remove(&next_start);

                if insert_end <= next_end {
                    acc = f(acc, InsertItem::Overlap(next_start..insert_end));
                    self.intervals
                        .insert(insert_start, insert_end.max(next_end));
                    return acc;
                }

                acc = f(acc, InsertItem::Overlap(next_start..next_end));
                start = next_end;
            }

            if start < insert_end {
                acc = f(acc, InsertItem::New(start..insert_end));
            }
            self.intervals.insert(insert_start, insert_end);
            acc
        }

        pub fn remove<U, F>(&mut self, interval: Range<T>, init: U, f: F) -> U
        where
            F: FnMut(U, RemoveItem<T>) -> U,
        {
            assert!(!interval.is_empty());

            let mut acc = init;
            let mut f = f;
            let remove_end = interval.end;
            let mut start = match self.intervals.range(..=interval.start).last() {
                Some((&prev_start, &prev_end)) if interval.start < prev_end => {
                    self.intervals.remove(&prev_start);

                    if prev_start < interval.start {
                        self.intervals.insert(prev_start, interval.start);
                    }

                    let overlap_end = prev_end.min(remove_end);
                    acc = f(acc, RemoveItem::Remove(interval.start..overlap_end));

                    if prev_end > remove_end {
                        self.intervals.insert(remove_end, prev_end);
                        return acc;
                    }
                    overlap_end
                }
                _ => interval.start,
            };

            while let Some((&next_start, &next_end)) = self.intervals.range(start..remove_end).next() {
                assert!(start <= next_start);
                assert!(next_start < remove_end);

                if start < next_start {
                    acc = f(acc, RemoveItem::Absent(start..next_start));
                }

                self.intervals.remove(&next_start);

                if next_end <= remove_end {
                    acc = f(acc, RemoveItem::Remove(next_start..next_end));
                    start = next_end;
                } else {
                    acc = f(acc, RemoveItem::Remove(next_start..remove_end));
                    self.intervals.insert(remove_end, next_end);
                    return acc;
                }
            }

            if start < remove_end {
                acc = f(acc, RemoveItem::Absent(start..remove_end));
            }

            acc
        }

        pub fn ge(&self, x: T) -> Option<Range<T>> {
            self.intervals
                .range(x..)
                .next()
                .map(|(&start, &end)| start..end)
        }

        pub fn le(&self, x: T) -> Option<Range<T>> {
            self.intervals
                .range(..=x)
                .last()
                .map(|(&start, &end)| start..end)
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
