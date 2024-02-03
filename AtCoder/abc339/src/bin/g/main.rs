use std::ops;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        q: usize,
    };

    let bucket_size = 4000; // (1..).take_while(|&s| s * s <= n).last().unwrap();
    let mut buckets = Vec::new();
    for chunk in a.chunks(bucket_size) {
        let elements = chunk.to_vec();
        buckets.push(Bucket::new(elements));
    }

    let mut ans = Vec::new();
    for _ in 0..q {
        input! {
            alpha: u64,
            beta: u64,
            gamma: u64,
        };
        let (l, r, x) = {
            let last = ans.last().copied().unwrap_or(0);
            (
                (alpha ^ last) as usize,
                (beta ^ last) as usize,
                (gamma ^ last) as u64,
            )
        };

        let mut sum = 0;
        let it = BucketRangeIter {
            bucket_size,
            element_range: (l - 1)..r,
        };
        for range in it {
            match range {
                BucketRangeType::Whole(i) => {
                    // log かかるので、できるだけここに入ってこないように bucket_size を大きめにする
                    let p = buckets[i].sorted_elements.partition_point(|&y| y <= x);
                    sum += buckets[i].sorted_elements_cumsum[p];
                }
                BucketRangeType::Part((i, r)) => {
                    for j in r {
                        if buckets[i].elements[j] <= x {
                            sum += buckets[i].elements[j];
                        }
                    }
                }
            }
        }
        ans.push(sum);
    }

    for ans in ans {
        println!("{}", ans);
    }
}

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
    elements: Vec<u64>,
    sorted_elements: Vec<u64>,
    sorted_elements_cumsum: Vec<u64>,
}

impl Bucket {
    fn new(elements: Vec<u64>) -> Self {
        let mut sorted_elements = elements.clone();
        sorted_elements.sort();
        let mut cumsum = vec![0; sorted_elements.len() + 1];
        for i in 1..cumsum.len() {
            cumsum[i] = cumsum[i - 1] + sorted_elements[i - 1];
        }
        Self {
            elements,
            sorted_elements,
            sorted_elements_cumsum: cumsum,
        }
    }
}
