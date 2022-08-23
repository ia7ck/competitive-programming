use proconio::input;
use proconio::marker::Usize1;
use std::ops::Range;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u8; n],
    };

    let bucket_size = (1..).take_while(|&s| s * s <= n).last().unwrap();

    #[derive(Debug)]
    struct Bucket {
        data: Data,
        lazy: Lazy,
        values: Vec<u8>,
    }
    #[derive(Debug)]
    struct Lazy {
        flip: bool,
    }
    #[derive(Debug)]
    struct Data {
        n0: usize,
        n1: usize,
        inv: usize,
    }
    let n_bucket = if n % bucket_size == 0 {
        n / bucket_size
    } else {
        n / bucket_size + 1
    };
    let mut buckets: Vec<Bucket> = (0..n_bucket)
        .map(|_| Bucket {
            data: Data {
                n0: 0,
                n1: 0,
                inv: 0,
            },
            lazy: Lazy { flip: false },
            values: Vec::new(),
        })
        .collect();

    for j in 0..n {
        let i = j / bucket_size;
        let bucket = &mut buckets[i];
        bucket.values.push(a[j]);
    }

    macro_rules! init {
        ($bucket: expr) => {
            let mut n0 = 0;
            let mut n1 = 0;
            let mut inv = 0;
            for j in 0..$bucket.values.len() {
                if $bucket.values[j] == 0 {
                    n0 += 1;
                    inv += n1;
                } else {
                    n1 += 1;
                }
            }
            $bucket.data = Data { n0, n1, inv };
        };
    }

    for i in 0..n_bucket {
        init!(buckets[i]);
    }

    #[derive(Debug)]
    enum Target {
        Large(usize),
        Small((usize, Range<usize>)),
    }
    let targets = |range: Range<usize>| -> Vec<Target> {
        let (l, r) = (range.start, range.end);
        assert!(l < r);
        let (il, ir) = (l / bucket_size, (r - 1) / bucket_size);
        if il == ir {
            return vec![Target::Small((
                il,
                (l - il * bucket_size)..(r - ir * bucket_size),
            ))];
        }

        let mut result = Vec::new();
        let mut j = l;
        loop {
            let i = j / bucket_size;
            if il == i {
                result.push(Target::Small((i, (l - il * bucket_size)..bucket_size)));
                j = (il + 1) * bucket_size;
            } else if il < i && i < ir {
                result.push(Target::Large(i));
                j += bucket_size;
            } else {
                assert_eq!(ir, i);
                result.push(Target::Small((i, 0..(r - ir * bucket_size))));
                break;
            }
        }
        result
    };

    macro_rules! push {
        ($bucket: expr) => {
            if $bucket.lazy.flip {
                for j in 0..$bucket.values.len() {
                    $bucket.values[j] = 1 - $bucket.values[j];
                }
                $bucket.lazy.flip = false;
            }
        };
    }

    for _ in 0..q {
        input! {
            t: u8,
            l: Usize1,
            r: Usize1,
        };

        if t == 1 {
            for t in targets(l..(r + 1)) {
                match t {
                    Target::Large(i) => {
                        buckets[i].data = Data {
                            n0: buckets[i].data.n1,
                            n1: buckets[i].data.n0,
                            inv: buckets[i].data.n0 * buckets[i].data.n1 - buckets[i].data.inv,
                        };
                        buckets[i].lazy = Lazy {
                            flip: !buckets[i].lazy.flip,
                        };
                    }
                    Target::Small((i, range)) => {
                        push!(buckets[i]);
                        for j in range {
                            buckets[i].values[j] = 1 - buckets[i].values[j];
                        }
                        init!(buckets[i]);
                    }
                }
            }
        } else {
            let mut n1 = 0;
            let mut inv = 0;
            for t in targets(l..(r + 1)) {
                match t {
                    Target::Large(i) => {
                        inv += buckets[i].data.inv + n1 * buckets[i].data.n0;
                        n1 += buckets[i].data.n1;
                    }
                    Target::Small((i, range)) => {
                        push!(buckets[i]);
                        for j in range {
                            if buckets[i].values[j] == 0 {
                                inv += n1;
                            } else {
                                n1 += 1;
                            }
                        }
                    }
                }
            }
            println!("{}", inv);
        }
    }
}
