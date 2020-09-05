struct ProconReader<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> ProconReader<R> {
    fn new(reader: R) -> Self {
        Self { reader }
    }
    fn get<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&byte| byte == b' ' || byte == b'\n' || byte == b'\r')
            .take_while(|&byte| byte != b' ' && byte != b'\n' && byte != b'\r')
            .collect::<Vec<_>>();
        std::str::from_utf8(&buf)
            .unwrap()
            .parse()
            .ok()
            .expect("Parse Error.")
    }
}

struct SegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    e: T,
    multiply: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Copy,
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
    fn fold(&self, range: std::ops::Range<usize>) -> T {
        self._fold(&range, 0, 0..self.n)
    }
    fn _fold(
        &self,
        range: &std::ops::Range<usize>,
        i: usize,
        i_range: std::ops::Range<usize>,
    ) -> T {
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

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();
    let a: Vec<usize> = (0..n).map(|_| rd.get()).collect();

    let inf = std::usize::MAX;
    let mut seg = SegmentTree::new(
        n,
        (inf, inf),
        |(a, i), (b, j)| {
            if a <= b {
                (a, i)
            } else {
                (b, j)
            }
        },
    );
    for i in 0..n {
        seg.update(i, (a[i], i));
    }
    for _ in 0..q {
        let t: usize = rd.get();
        let l: usize = rd.get();
        let r: usize = rd.get();
        match t {
            1 => {
                let (al, _) = seg.get(l - 1);
                let (ar, _) = seg.get(r - 1);
                seg.update(l - 1, (ar, l - 1));
                seg.update(r - 1, (al, r - 1));
            }
            2 => {
                let (_, i) = seg.fold((l - 1)..r);
                println!("{}", i + 1);
            }
            _ => unreachable!(),
        }
    }
}
