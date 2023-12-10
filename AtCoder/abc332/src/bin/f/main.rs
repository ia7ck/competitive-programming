use std::ops;

use mod_int::ModInt998244353;
use proconio::{input, marker::Usize1};

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

type Mint = ModInt998244353;

#[derive(Debug)]
struct Bucket {
    #[allow(unused)]
    acc: Acc,
    lazy: Lazy,
    elements: Vec<Mint>,
}

#[derive(Debug)]
struct Acc {}

#[derive(Debug)]
struct Lazy {
    // a * x + b
    a: Mint,
    b: Mint,
}

impl Default for Lazy {
    fn default() -> Self {
        Self {
            a: Mint::new(1),
            b: Mint::new(0),
        }
    }
}

impl Bucket {
    fn new(elements: Vec<Mint>) -> Self {
        let mut bucket = Self {
            acc: Acc {}, // default
            lazy: Lazy::default(),
            elements,
        };
        bucket.update_acc(); // ★
        bucket
    }

    // 遅延をバケット内の各要素に伝搬する
    fn push_lazy(&mut self) {
        for j in 0..self.elements.len() {
            self.elements[j] = self.lazy.a * self.elements[j] + self.lazy.b;
        }
        self.lazy = Lazy::default(); // ★
    }

    // バケット内の各要素からデータを更新する
    fn update_acc(&mut self) {}
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    };

    let bucket_size = (1..).take_while(|&s| s * s <= n).last().unwrap();
    let mut buckets = Vec::new();
    for chunk in a.chunks(bucket_size) {
        let elements = chunk.iter().map(|&x| Mint::new(x)).collect::<Vec<_>>();
        buckets.push(Bucket::new(elements));
    }

    for _ in 0..m {
        input! {
            l: Usize1,
            r: Usize1,
            x: i64,
        };

        let p = Mint::from(r - l + 1).inv();

        let it = BucketRangeIter {
            bucket_size,
            element_range: l..(r + 1),
        };
        for range in it {
            match range {
                BucketRangeType::Whole(i) => {
                    let c = Mint::new(1) - p;
                    let d = p * x;
                    // (a * x + b) * c + d = (a * c) * x + (b * c + d)
                    buckets[i].lazy = Lazy {
                        a: buckets[i].lazy.a * c,
                        b: buckets[i].lazy.b * c + d,
                    };
                }
                BucketRangeType::Part((i, r)) => {
                    buckets[i].push_lazy(); // ★
                    for j in r {
                        buckets[i].elements[j] =
                            buckets[i].elements[j] * (Mint::new(1) - p) + p * x;
                    }
                    buckets[i].update_acc(); // ★
                }
            }
        }
    }

    let mut ans = Vec::new();
    for bucket in &mut buckets {
        bucket.push_lazy();
        for elem in &bucket.elements {
            ans.push(elem.val());
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
