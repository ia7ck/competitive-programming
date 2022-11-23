use std::ops;

use proconio::{input, marker::Usize1};
use rustc_hash::FxHashMap;

#[derive(Debug)]
enum BucketRangeType {
    Whole(usize),
    Part((usize, ops::Range<usize>)),
}

#[derive(Debug)]
struct BucketRangeIter {
    bucket_size: usize,
    value_range: ops::Range<usize>,
}

impl Iterator for BucketRangeIter {
    type Item = BucketRangeType;

    fn next(&mut self) -> Option<Self::Item> {
        let (l, r) = (self.value_range.start, self.value_range.end);
        if l == r {
            return None;
        }
        let (il, ir) = (l / self.bucket_size, r / self.bucket_size);
        if il == ir {
            self.value_range = r..r;
            return Some(BucketRangeType::Part((
                il,
                (l % self.bucket_size)..(r % self.bucket_size),
            )));
        }
        self.value_range = ((il + 1) * self.bucket_size)..r;
        if l % self.bucket_size == 0 {
            Some(BucketRangeType::Whole(il))
        } else {
            Some(BucketRangeType::Part((
                il,
                (l % self.bucket_size)..self.bucket_size,
            )))
        }
    }
}

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        q: usize,
    };

    let mut freq = FxHashMap::default();
    for &x in &a {
        *freq.entry(x).or_insert(0_u64) += 1;
    }
    let mut ans = 0;
    for &f in freq.values() {
        ans += f * (f - 1) / 2;
    }

    let bucket_size = (1..).take_while(|&s| s * s <= n).last().unwrap();

    #[derive(Debug)]
    struct Bucket {
        data: Data,
        lazy: Lazy,
        values: Vec<u32>,
    }
    #[derive(Debug)]
    struct Data {}
    #[derive(Debug)]
    struct Lazy {
        y: Option<u32>,
    }

    let mut buckets = Vec::new();
    for chunk in a.chunks(bucket_size) {
        buckets.push(Bucket {
            data: Data {},
            lazy: Lazy { y: None },
            values: chunk.to_vec(),
        });
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
            x: u32,
        };

        let it = BucketRangeIter {
            bucket_size,
            value_range: l..(r + 1),
        };

        let mut del = FxHashMap::default();

        for range in it {
            match range {
                BucketRangeType::Whole(i) => {
                    if let Some(y) = buckets[i].lazy.y {
                        *del.entry(y).or_insert(0) += buckets[i].values.len() as u64;
                    } else {
                        for v in buckets[i].values.iter_mut() {
                            *del.entry(*v).or_insert(0) += 1;
                            *v = x;
                        }
                    }
                    buckets[i].lazy.y = Some(x);
                }
                BucketRangeType::Part((i, value_range)) => {
                    if let Some(y) = buckets[i].lazy.y.take() {
                        for v in buckets[i].values.iter_mut() {
                            *v = y;
                        }
                    }
                    for j in value_range {
                        *del.entry(buckets[i].values[j]).or_insert(0) += 1;
                        buckets[i].values[j] = x;
                    }
                }
            }
        }
        for (v, c) in del {
            // f * (f - 1) / 2 - (f - c) * (f - c - 1) / 2
            let f = freq.get_mut(&v).unwrap();
            assert!(*f >= c);
            assert!(*f * (*f - 1) / 2 >= (*f - c) * (*f - c).saturating_sub(1) / 2);
            assert!(ans >= *f * (*f - 1) / 2 - (*f - c) * (*f - c).saturating_sub(1) / 2);
            ans -= *f * (*f - 1) / 2 - (*f - c) * (*f - c).saturating_sub(1) / 2;
            *f -= c;
            if *f == 0 {
                freq.remove(&v);
            }
        }

        let c = (r - l + 1) as u64;
        let f = freq.entry(x).or_insert(0);
        // (f + c) * (f + c - 1) / 2 - f * (f - 1) / 2
        assert!((*f + c) * (*f + c - 1) / 2 >= *f * f.saturating_sub(1) / 2);
        ans += (*f + c) * (*f + c - 1) / 2 - *f * f.saturating_sub(1) / 2;
        *f += c as u64;

        println!("{}", ans);
    }
}
