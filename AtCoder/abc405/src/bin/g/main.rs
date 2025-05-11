use mod_int::ModInt998244353;
use proconio::{input, marker::Usize1};

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        queries: [(Usize1, Usize1, usize); q],
    };

    let q_sqrt = f64::sqrt(q as f64) as usize;
    let mut ord = (0..q).collect::<Vec<_>>();
    ord.sort_by_key(|&i| {
        let (l, r, _) = queries[i];
        let j = l * q_sqrt / n;
        (j, if j % 2 == 0 { r } else { n - r })
    });

    let mut factorials = vec![Mint::new(1); n + 1];
    for i in 1..=n {
        factorials[i] = factorials[i - 1] * Mint::from(i);
    }
    let mut inv_factorials = vec![Mint::new(1); n + 1];
    inv_factorials[n] = factorials[n].inv();
    for i in (1..n).rev() {
        inv_factorials[i] = inv_factorials[i + 1] * Mint::from(i + 1);
    }

    // let bucket_size = f64::sqrt(n as f64) as usize;
    let bucket_size = 400;

    struct S {
        buckets: Vec<Bucket>,
    }

    let add = |state: &mut S, i: usize| {
        let (bi, bj) = (a[i] / bucket_size, a[i] % bucket_size);
        let old_c = state.buckets[bi].count[bj];
        let new_c = old_c + 1;
        state.buckets[bi].count[bj] = new_c;
        state.buckets[bi].total += 1;
        state.buckets[bi].inv_fact_prod *= factorials[old_c] * inv_factorials[new_c];
    };

    let remove = |state: &mut S, i: usize| {
        let (bi, bj) = (a[i] / bucket_size, a[i] % bucket_size);
        let old_c = state.buckets[bi].count[bj];
        let new_c = old_c - 1;
        state.buckets[bi].count[bj] = new_c;
        state.buckets[bi].total -= 1;
        state.buckets[bi].inv_fact_prod *= factorials[old_c] * inv_factorials[new_c];
    };

    let mut state = {
        let mut count = vec![0; n + 1];
        for &x in &a {
            count[x] += 1;
        }
        let mut buckets = Vec::new();
        for chunk in count.chunks(bucket_size) {
            buckets.push(Bucket::new(chunk, &inv_factorials));
        }
        S { buckets }
    };

    let mut ans = vec![Mint::new(0); q];
    let (mut left, mut right) = (0, n - 1);
    for i in ord {
        let (l, r, x) = queries[i];
        while l < left {
            left -= 1;
            add(&mut state, left);
        }
        while right < r {
            right += 1;
            add(&mut state, right);
        }
        while left < l {
            remove(&mut state, left);
            left += 1;
        }
        while r < right {
            remove(&mut state, right);
            right -= 1;
        }

        let it = BucketRangeIter {
            bucket_size,
            element_range: 1..x,
        };
        let mut total = 0;
        let mut denom = Mint::new(1);
        for range in it {
            match range {
                BucketRangeType::Whole(bi) => {
                    total += state.buckets[bi].total;
                    denom *= state.buckets[bi].inv_fact_prod;
                }
                BucketRangeType::Part((bi, r)) => {
                    for bj in r {
                        let c = state.buckets[bi].count[bj];
                        total += c;
                        denom *= inv_factorials[c];
                    }
                }
            }
        }

        ans[i] = factorials[total] * denom;
    }

    for ans in ans {
        println!("{}", ans.val());
    }
}

#[derive(Debug)]
struct Bucket {
    count: Vec<usize>,
    total: usize,        // sum(count[*])
    inv_fact_prod: Mint, // Π 1/(count[i]!)
}

impl Bucket {
    fn new(chunk: &[usize], inv_fs: &[Mint]) -> Self {
        Self {
            count: chunk.to_vec(),
            total: chunk.iter().sum(),
            inv_fact_prod: chunk.iter().fold(Mint::new(1), |acc, &c| acc * inv_fs[c]),
        }
    }
}

use std::{ops, vec};

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
