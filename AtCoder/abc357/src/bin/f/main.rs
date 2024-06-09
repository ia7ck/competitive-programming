use mod_int::ModInt998244353;
use proconio::{input, marker::Usize1};

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        b: [u64; n],
    };

    let ab = a.into_iter().zip(b.into_iter()).collect::<Vec<_>>();

    let bucket_size = 1000;
    let mut buckets = Vec::new();
    for chunk in ab.chunks(bucket_size) {
        let elements = chunk
            .iter()
            .map(|&(a, b)| (Mint::from(a), Mint::from(b)))
            .collect::<Vec<_>>();
        buckets.push(Bucket::new(elements));
    }

    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                l: Usize1,
                r: Usize1,
                x: u64,
            };
            let x = Mint::from(x);
            let it = BucketRangeIter {
                bucket_size,
                element_range: l..(r + 1),
            };
            for range in it {
                match range {
                    BucketRangeType::Whole(i) => {
                        buckets[i].add_a += x;
                        let len = buckets[i].elements.len();
                        buckets[i].sum_a += x * len;
                        let sum_b = buckets[i].sum_b;
                        buckets[i].sum += x * sum_b;
                    },
                    BucketRangeType::Part((i, r)) => {
                        buckets[i].push_lazy();
                        for j in r {
                            buckets[i].elements[j].0 += x;
                        }
                        buckets[i].update_acc();
                    },
                }
            }
        } else if op == 2 {
            input! {
                l: Usize1,
                r: Usize1,
                x: u64,
            };
            let x = Mint::from(x);
            let it = BucketRangeIter {
                bucket_size,
                element_range: l..(r + 1),
            };
            for range in it {
                match range {
                    BucketRangeType::Whole(i) => {
                        buckets[i].add_b += x;
                        let len = buckets[i].elements.len();
                        buckets[i].sum_b += x * len;
                        let sum_a = buckets[i].sum_a;
                        buckets[i].sum += x * sum_a;
                    },
                    BucketRangeType::Part((i, r)) => {
                        buckets[i].push_lazy();
                        for j in r {
                            buckets[i].elements[j].1 += x;
                        }
                        buckets[i].update_acc();
                    },
                }
            }
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };
            let mut ans = Mint::new(0);
            let it = BucketRangeIter {
                bucket_size,
                element_range: l..(r + 1),
            };
            for range in it {
                match range {
                    BucketRangeType::Whole(i) => {
                        ans += buckets[i].sum;
                    },
                    BucketRangeType::Part((i, r)) => {
                        buckets[i].push_lazy();
                        for j in r {
                            ans += buckets[i].elements[j].0 * buckets[i].elements[j].1;
                        }
                    },
                }
            }
            println!("{}", ans.val());
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

struct Bucket {
    elements: Vec<(Mint, Mint)>, // a, b
    // acc
    sum: Mint,
    sum_a: Mint,
    sum_b: Mint,
    // lazy
    add_a: Mint,
    add_b: Mint,
}

impl Bucket {
    fn new(elements: Vec<(Mint, Mint)>) -> Self {
        let mut sum = Mint::new(0);
        let mut sum_a = Mint::new(0);
        let mut sum_b = Mint::new(0);
        for &(a, b) in &elements {
            sum += a * b;
            sum_a += a;
            sum_b += b;
        }
        Self {
            elements,
            sum,
            sum_a,
            sum_b,
            add_a: Mint::new(0),
            add_b: Mint::new(0),
        }
    }

    fn push_lazy(&mut self) {
        for j in 0..self.elements.len() {
            self.elements[j].0 += self.add_a;
            self.elements[j].1 += self.add_b;
        }
        self.add_a = Mint::new(0);
        self.add_b = Mint::new(0);
        self.update_acc();
    }

    fn update_acc(&mut self) {
        self.sum = Mint::new(0);
        self.sum_a = Mint::new(0);
        self.sum_b = Mint::new(0);
        for j in 0..self.elements.len() {
            self.sum += self.elements[j].0 * self.elements[j].1;
            self.sum_a += self.elements[j].0;
            self.sum_b += self.elements[j].1;
        }
    }
}
