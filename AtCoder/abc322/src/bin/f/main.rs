use std::ops;

use proconio::{
    input,
    marker::{Bytes, Usize1},
};

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

// ✄--------------------------------✄

#[derive(Debug)]
struct Bucket {
    data: Data,
    lazy: Lazy,
    values: Vec<u8>, // 0/1
}
#[derive(Debug)]
struct Data {
    any_0: usize,
    any_1: usize,
    left_0: usize,
    right_0: usize,
    left_1: usize,
    right_1: usize,
}
#[derive(Debug)]
struct Lazy {
    flip: bool,
}

impl Bucket {
    fn new(values: Vec<u8>) -> Self {
        let mut bucket = Self {
            // default
            data: Data {
                any_0: 0,
                any_1: 0,
                left_0: 0,
                right_0: 0,
                left_1: 0,
                right_1: 0,
            },
            lazy: Lazy { flip: false },
            values,
        };
        bucket.update_data();
        bucket
    }

    // 遅延をバケット内の各要素に伝搬する
    fn push_lazy(&mut self) {
        if self.lazy.flip {
            for j in 0..self.values.len() {
                self.values[j] = 1 - self.values[j];
            }
        }
        self.lazy = Lazy { flip: false };
    }

    // バケット内の各要素からデータを更新する
    fn update_data(&mut self) {
        let (mut any_0, mut len_0) = (0, 0);
        let (mut any_1, mut len_1) = (0, 0);
        for j in 0..self.values.len() {
            if self.values[j] == 0 {
                len_0 += 1;
                len_1 = 0;
            } else {
                len_0 = 0;
                len_1 += 1;
            }
            any_0 = any_0.max(len_0);
            any_1 = any_1.max(len_1);
        }

        let left_0 = self.values.iter().take_while(|&&v| v == 0).count();
        let right_0 = self.values.iter().rev().take_while(|&&v| v == 0).count();
        let left_1 = self.values.iter().take_while(|&&v| v == 1).count();
        let right_1 = self.values.iter().rev().take_while(|&&v| v == 1).count();
        self.data = Data {
            any_0,
            any_1,
            left_0,
            right_0,
            left_1,
            right_1,
        };
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Bytes,
    };

    let bucket_size = (1..).take_while(|&s| s * s <= n).last().unwrap();
    let mut buckets = Vec::new();
    for chunk in s.chunks(bucket_size) {
        let values = chunk.iter().map(|x| x - b'0').collect::<Vec<_>>();
        let b = Bucket::new(values);
        buckets.push(b);
    }

    for _ in 0..q {
        input! {
            op: u8,
            l: Usize1,
            r: Usize1,
        };

        let it = BucketRangeIter {
            bucket_size,
            value_range: l..(r + 1),
        };

        if op == 1 {
            for range in it {
                match range {
                    BucketRangeType::Whole(i) => {
                        buckets[i].data = Data {
                            any_0: buckets[i].data.any_1,
                            any_1: buckets[i].data.any_0,
                            left_0: buckets[i].data.left_1,
                            right_0: buckets[i].data.right_1,
                            left_1: buckets[i].data.left_0,
                            right_1: buckets[i].data.right_0,
                        };
                        buckets[i].lazy.flip = !buckets[i].lazy.flip;
                    }
                    BucketRangeType::Part((i, r)) => {
                        buckets[i].push_lazy();
                        for j in r {
                            buckets[i].values[j] = 1 - buckets[i].values[j];
                        }
                        buckets[i].update_data();
                    }
                }
            }
        } else {
            let mut cur = 0;
            let mut ans = 0;
            for range in it {
                match range {
                    BucketRangeType::Whole(i) => {
                        ans = ans.max(buckets[i].data.any_1);

                        cur += buckets[i].data.left_1;
                        ans = ans.max(cur);
                        if buckets[i].data.left_1 < buckets[i].values.len() {
                            cur = buckets[i].data.right_1;
                            ans = ans.max(cur);
                        }
                    }
                    BucketRangeType::Part((i, r)) => {
                        buckets[i].push_lazy();
                        for j in r {
                            if buckets[i].values[j] == 1 {
                                cur += 1;
                                ans = ans.max(cur);
                            } else {
                                cur = 0;
                            }
                        }
                    }
                }
            }
            println!("{}", ans);
        }
    }
}
