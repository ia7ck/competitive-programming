use procon_reader::ProconReader;
use std::ops::Range;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let mut ys = vec![vec![]; n];
    for _ in 0..m {
        let x: usize = rd.get();
        let y: usize = rd.get();
        ys[x - 1].push(y - 1);
    }

    for x in 0..n {
        ys[x].sort();
    }
    let mut seg = SegmentTree::new(n, 0usize, |l1, l2| l1.max(l2));
    for x in 0..n {
        for &y in ys[x].iter().rev() {
            let cur = seg.get(y);
            seg.update(y, cur.max(1 + seg.fold(0..y)));
        }
    }
    let ans = seg.fold(0..n);
    println!("{}", ans);
}

struct SegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    e: T,
    multiply: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    fn new(n: usize, e: T, multiply: F) -> Self {
        let n = n.next_power_of_two();
        Self {
            n,
            dat: vec![e; n * 2 - 1],
            e,
            multiply,
        }
    }
    fn get(&self, i: usize) -> T {
        self.dat[i + self.n - 1]
    }
    fn update(&mut self, i: usize, x: T) {
        let mut k = i + self.n - 1;
        self.dat[k] = x;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.multiply)(self.dat[k * 2 + 1], self.dat[k * 2 + 2]);
        }
    }
    fn fold(&self, range: Range<usize>) -> T {
        self._fold(&range, 0, 0..self.n)
    }
    fn _fold(&self, range: &Range<usize>, i: usize, i_range: Range<usize>) -> T {
        if range.end <= i_range.start || i_range.end <= range.start {
            return self.e;
        }
        if range.start <= i_range.start && i_range.end <= range.end {
            return self.dat[i];
        }
        let m = (i_range.start + i_range.end) / 2;
        (self.multiply)(
            self._fold(range, i * 2 + 1, i_range.start..m),
            self._fold(range, i * 2 + 2, m..i_range.end),
        )
    }
}
