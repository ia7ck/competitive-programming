use proconio::input;
use std::ops::Range;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(u64, u64); n],
    }
    let mo = 998244353;
    let mut seg = SegmentTree::new(n, &(1, 0), |(a, b), (c, d)| {
        // c * (ax + b) + d
        // = cax + cb + d
        (c * a % mo, (c * b % mo + d) % mo)
    });
    for (i, &(a, b)) in ab.iter().enumerate() {
        seg.update(i, (a, b));
    }
    for _ in 0..q {
        input! {
            t: u8,
        };
        if t == 0 {
            input! {
                p: usize,
                c: u64,
                d: u64,
            }
            seg.update(p, (c, d));
        } else {
            input! {
                l: usize,
                r: usize,
                x: u64,
            }
            let (a, b) = seg.fold(l..r);
            let ans = a * x % mo + b;
            println!("{}", ans % mo);
        }
    }
}

pub struct SegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    e: T,
    multiply: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    pub fn new(n: usize, e: &T, multiply: F) -> Self {
        let n = n.next_power_of_two();
        Self {
            n,
            dat: vec![e.clone(); n * 2 - 1],
            e: e.clone(),
            multiply,
        }
    }
    pub fn get(&self, i: usize) -> &T {
        &self.dat[i + self.n - 1]
    }
    pub fn update(&mut self, i: usize, x: T) {
        let mut k = i + self.n - 1;
        self.dat[k] = x;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.multiply)(&self.dat[k * 2 + 1], &self.dat[k * 2 + 2]);
        }
    }
    pub fn fold(&self, range: Range<usize>) -> T {
        self._fold(&range, 0, 0..self.n)
    }
    fn _fold(&self, range: &Range<usize>, i: usize, i_range: Range<usize>) -> T {
        if range.end <= i_range.start || i_range.end <= range.start {
            return self.e.clone();
        }
        if range.start <= i_range.start && i_range.end <= range.end {
            return self.dat[i].clone();
        }
        let m = (i_range.start + i_range.end) / 2;
        (self.multiply)(
            &self._fold(range, i * 2 + 1, i_range.start..m),
            &self._fold(range, i * 2 + 2, m..i_range.end),
        )
    }
}
