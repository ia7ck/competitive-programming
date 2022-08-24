use std::ops::Range;

use proconio::{input, marker::Usize1};

#[derive(Debug)]
enum BucketRangeType {
    Whole(usize),
    Part((usize, Range<usize>)),
}

#[derive(Debug)]
struct BucketRangeIter {
    bucket_size: usize,
    value_range: Range<usize>,
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
        q: usize,
        a: [usize; n],
    };

    let bucket_size = (1..).take_while(|&s| s * s <= n).last().unwrap();

    #[derive(Debug)]
    struct Bucket {
        data: Data,
        lazy: Lazy,
        values: Vec<usize>,
    }
    #[derive(Debug)]
    struct Data {
        freq: [usize; 3], // 0, 1, 2
        inv: [usize; 3],  // 10, 20, 21
    }
    #[derive(Debug)]
    struct Lazy {
        map: [usize; 3],
    }

    let mut buckets = Vec::new();
    for chunk in a.chunks(bucket_size) {
        buckets.push(Bucket {
            data: Data {
                freq: [0, 0, 0],
                inv: [0, 0, 0],
            },
            lazy: Lazy { map: [0, 1, 2] },
            values: chunk.to_vec(),
        });
    }

    macro_rules! push {
        ($bucket: expr) => {
            for j in 0..$bucket.values.len() {
                $bucket.values[j] = $bucket.lazy.map[$bucket.values[j]];
            }
            $bucket.lazy = Lazy { map: [0, 1, 2] };
        };
    }

    macro_rules! update {
        ($bucket: expr) => {
            let mut freq = [0, 0, 0];
            let (mut n_10, mut n_20, mut n_21) = (0, 0, 0);
            for &v in &$bucket.values {
                if v == 0 {
                    n_10 += freq[1];
                    n_20 += freq[2];
                } else if v == 1 {
                    n_21 += freq[2];
                }
                freq[v] += 1;
            }
            $bucket.data = Data {
                freq,
                inv: [n_10, n_20, n_21],
            };
        };
    }

    for i in 0..buckets.len() {
        update!(buckets[i]);
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
            let mut freq = [0, 0, 0];
            let mut inv = 0;
            for range in it {
                match range {
                    BucketRangeType::Whole(i) => {
                        let bucket = &buckets[i];
                        inv += bucket.data.inv.iter().sum::<usize>()
                            + freq[1] * bucket.data.freq[0]
                            + freq[2] * bucket.data.freq[0]
                            + freq[2] * bucket.data.freq[1];
                        freq[0] += bucket.data.freq[0];
                        freq[1] += bucket.data.freq[1];
                        freq[2] += bucket.data.freq[2];
                    }
                    BucketRangeType::Part((i, value_range)) => {
                        push!(buckets[i]);
                        for j in value_range {
                            let v = buckets[i].values[j];
                            if v == 0 {
                                inv += freq[1];
                                inv += freq[2];
                            } else if v == 1 {
                                inv += freq[2];
                            }
                            freq[v] += 1;
                        }
                    }
                }
            }
            println!("{}", inv);
        } else {
            input! {
                map: [usize; 3],
            };
            for range in it {
                match range {
                    BucketRangeType::Whole(i) => {
                        let bucket = &mut buckets[i];
                        let mut freq = [0, 0, 0];
                        freq[map[0]] += bucket.data.freq[0];
                        freq[map[1]] += bucket.data.freq[1];
                        freq[map[2]] += bucket.data.freq[2];
                        let mut pair = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
                        pair[map[1]][map[0]] += bucket.data.inv[0];
                        pair[map[2]][map[0]] += bucket.data.inv[1];
                        pair[map[2]][map[1]] += bucket.data.inv[2];
                        pair[map[0]][map[1]] +=
                            bucket.data.freq[0] * bucket.data.freq[1] - bucket.data.inv[0];
                        pair[map[0]][map[2]] +=
                            bucket.data.freq[0] * bucket.data.freq[2] - bucket.data.inv[1];
                        pair[map[1]][map[2]] +=
                            bucket.data.freq[1] * bucket.data.freq[2] - bucket.data.inv[2];
                        bucket.data = Data {
                            freq,
                            inv: [pair[1][0], pair[2][0], pair[2][1]],
                        };
                        bucket.lazy = Lazy {
                            map: [
                                map[bucket.lazy.map[0]],
                                map[bucket.lazy.map[1]],
                                map[bucket.lazy.map[2]],
                            ],
                        };
                    }
                    BucketRangeType::Part((i, value_range)) => {
                        push!(buckets[i]);
                        for j in value_range {
                            buckets[i].values[j] = map[buckets[i].values[j]];
                        }
                        update!(buckets[i]);
                    }
                }
            }
        }
    }
}
