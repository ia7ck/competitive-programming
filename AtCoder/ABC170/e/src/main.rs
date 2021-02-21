fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();

    let m = 200_000;

    let mut rating = vec![0; n + 1];
    let mut pos = vec![0; n + 1];
    use std::collections::BTreeMap;
    // freqs[i][j]: 幼稚園 i にいるレート j の人数
    let mut freqs = vec![BTreeMap::new(); m + 1];
    for i in 1..=n {
        let a: u32 = rd.get();
        let b: usize = rd.get();
        rating[i] = a;
        pos[i] = b;
        let e = freqs[b].entry(a).or_insert(0);
        *e += 1;
    }
    // highs[i]: 幼稚園 i にいる最高レート
    let inf = std::u32::MAX;
    let mut high = SegmentTree::new(m + 1, inf, |x, y| x.min(y));
    for i in 1..=m {
        if let Some((&max, _)) = freqs[i].iter().last() {
            high.update(i, max);
        }
    }
    for _ in 0..q {
        let c: usize = rd.get();
        let d: usize = rd.get();
        let r = rating[c];
        let p = pos[c]; // p -> d
        let f = freqs[p].get_mut(&r).unwrap();
        *f -= 1;
        if *f == 0 {
            freqs[p].remove(&r);
            if let Some((&max, _)) = freqs[p].iter().last() {
                high.update(p, max);
            } else {
                // 幼稚園 p に誰もいなくなった場合
                high.update(p, inf);
            }
        }
        let f = freqs[d].entry(r).or_insert(0);
        *f += 1;
        if *f == 1 {
            // 園児 c が幼稚園 d 内で最強とは限らない
            let (&max, _) = freqs[d].iter().last().unwrap();
            high.update(d, max);
        }
        pos[c] = d;
        let ans = high.fold(1..(m + 1)); // 1..=m
        println!("{}", ans);
    }
}

struct SegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    e: T,
    multiply: F,
}

#[allow(dead_code)]
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

pub struct ProconReader<R> {
    r: R,
    l: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            l: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.l.len()); // remain some character
        assert_ne!(&self.l[self.i..=self.i], " ");
        let rest = &self.l[self.i..];
        let len = rest.find(' ').unwrap_or_else(|| rest.len());
        // parse self.l[self.i..(self.i + len)]
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|e| panic!("{:?}, attempt to read `{}`", e, rest));
        self.i += len;
        val
    }
    fn skip_blanks(&mut self) {
        loop {
            match self.l[self.i..].find(|ch| ch != ' ') {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    let mut buf = String::new();
                    let num_bytes = self
                        .r
                        .read_line(&mut buf)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = buf
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
                    self.i = 0;
                }
            }
        }
    }
    pub fn get_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.get()).collect()
    }
    pub fn get_chars(&mut self) -> Vec<char> {
        self.get::<String>().chars().collect()
    }
}
