use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        q: usize,
        queries: [(Usize1, Usize1, u64); q],
    };

    let bucket_size = 1000;
    let mut buckets = Vec::new();
    for chunk in a.chunks(bucket_size) {
        buckets.push(Bucket::new(chunk));
    }

    for (l, r, k) in queries {
        let mut ans = 0;
        let it = BucketRangeIter {
            bucket_size,
            element_range: l..(r + 1),
        };
        for range in it {
            match range {
                BucketRangeType::Whole(i) => {
                    if buckets[i].positive == 0 {
                        assert!(buckets[i].min.is_none());
                        buckets[i].subtract += k;
                    } else if buckets[i]
                        .min
                        .is_some_and(|min| min - buckets[i].subtract <= k)
                    {
                        let sub = buckets[i].subtract;
                        let mut new_min = u64::MAX;
                        let mut new_positive = 0;
                        for x in &mut buckets[i].elements {
                            if let Some(x) = x {
                                *x = x.saturating_sub(sub);

                                let k = k.min(*x);
                                *x -= k;
                                ans += k;

                                if *x > 0 {
                                    new_min = new_min.min(*x);
                                    new_positive += 1;
                                }
                            }
                        }
                        buckets[i].min = if new_min == u64::MAX {
                            None
                        } else {
                            Some(new_min)
                        };
                        buckets[i].positive = new_positive;
                        buckets[i].subtract = 0;
                    } else {
                        ans += k * buckets[i].positive as u64;
                        buckets[i].subtract += k;
                    }
                }
                BucketRangeType::Part((i, r)) => {
                    let sub = buckets[i].subtract;
                    let mut new_min = u64::MAX;
                    let mut new_positive = 0;
                    for (j, x) in buckets[i].elements.iter_mut().enumerate() {
                        if let Some(x) = x {
                            *x = x.saturating_sub(sub);

                            if r.contains(&j) {
                                let k = k.min(*x);
                                *x -= k;
                                ans += k;
                            }

                            if *x > 0 {
                                new_min = new_min.min(*x);
                                new_positive += 1;
                            }
                        }
                    }
                    buckets[i].min = if new_min == u64::MAX {
                        None
                    } else {
                        Some(new_min)
                    };
                    buckets[i].positive = new_positive;
                    buckets[i].subtract = 0;
                }
            }
        }
        println!("{}", ans);
    }
}

#[derive(Debug)]
struct Bucket {
    elements: Vec<Option<u64>>,
    min: Option<u64>, // min {x | x.is_some() && x in elements}
    positive: usize,  // #{ x.is_some() && x in elements }
    subtract: u64,
}

impl Bucket {
    fn new(chunk: &[u64]) -> Self {
        Self {
            elements: chunk.iter().map(|&x| Some(x)).collect(),
            min: chunk.iter().min().copied(),
            positive: chunk.len(),
            subtract: 0,
        }
    }
}

use std::ops;

#[derive(Debug)]
enum BucketRangeType {
    Whole(usize),
    Part((usize, ops::Range<usize>)),
}

#[derive(Debug)]
struct BucketRangeIter {
    bucket_size: usize,
    element_range: ops::Range<usize>,
}

impl Iterator for BucketRangeIter {
    type Item = BucketRangeType;

    fn next(&mut self) -> Option<Self::Item> {
        let (l, r) = (self.element_range.start, self.element_range.end);
        if l == r {
            return None;
        }
        let (il, ir) = (l / self.bucket_size, r / self.bucket_size);
        if il == ir {
            self.element_range = r..r;
            return Some(BucketRangeType::Part((
                il,
                (l % self.bucket_size)..(r % self.bucket_size),
            )));
        }
        self.element_range = ((il + 1) * self.bucket_size)..r;
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
