fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let r: u64 = rd.get();
    let t: u64 = rd.get();
    let mut edges = Vec::new();
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        let c: u64 = rd.get();
        edges.push((a - 1, b - 1, c * r * t));
    }
    let mut g = vec![vec![]; n]; // usagi
    let mut h = vec![vec![]; n]; // kame
    for (a, b, c) in edges {
        g[a].push(Edge { to: b, cost: c / r });
        g[b].push(Edge { to: a, cost: c / r });
        h[a].push(Edge { to: b, cost: c / t });
        h[b].push(Edge { to: a, cost: c / t });
    }
    let mut ans = 0;
    for a in 0..n {
        let (usagi, _) = dijkstra(&g, a);
        let (kame, _) = dijkstra(&h, a);
        let usagi: Vec<u64> = usagi.into_iter().map(|d| d.unwrap()).collect();
        let mut kame: Vec<u64> = kame.into_iter().map(|d| d.unwrap()).collect();
        kame.sort();
        let kame: Vec<u64> = kame.into_iter().skip(1).collect();
        for (b, d) in usagi.iter().enumerate() {
            if b == a {
                continue;
            }
            let kame_win = kame.lower_bound(d);
            ans += kame_win;
            if r < t {
                assert!(kame_win >= 1);
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
    fn split_by(
        &self,
        x: &T,
    ) -> (
        std::ops::Range<usize>,
        std::ops::Range<usize>,
        std::ops::Range<usize>,
    );
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        if self[0].ge(x) {
            return 0;
        }
        let mut lf = 0;
        let mut rg = self.len();
        while rg - lf > 1 {
            let md = (rg + lf) / 2;
            if self[md].lt(x) {
                lf = md;
            } else {
                rg = md;
            }
        }
        rg
    }

    fn upper_bound(&self, x: &T) -> usize {
        if self[0].gt(x) {
            return 0;
        }
        let mut lf = 0;
        let mut rg = self.len();
        while rg - lf > 1 {
            let md = (rg + lf) / 2;
            if self[md].le(x) {
                lf = md;
            } else {
                rg = md;
            }
        }
        rg
    }

    fn split_by(
        &self,
        x: &T,
    ) -> (
        std::ops::Range<usize>,
        std::ops::Range<usize>,
        std::ops::Range<usize>,
    ) {
        let i = self.lower_bound(x);
        let j = self.upper_bound(x);
        (0..i, i..j, j..self.len())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub to: usize,
    pub cost: u64,
}

#[allow(clippy::many_single_char_names)]
pub fn dijkstra(g: &[Vec<Edge>], s: usize) -> (Vec<Option<u64>>, Vec<Option<usize>>) {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = g.len();
    let mut dist = vec![None; n];
    let mut q = BinaryHeap::new();
    let mut prev = vec![None; n];
    dist[s] = Some(0);
    q.push((Reverse(0), s));
    while let Some((Reverse(d), v)) = q.pop() {
        if dist[v].map_or(false, |min| d > min) {
            continue;
        }
        for e in &g[v] {
            let next_d = d + e.cost;
            if dist[e.to].map_or(true, |cur_d| next_d < cur_d) {
                dist[e.to] = Some(next_d);
                prev[e.to] = Some(v);
                q.push((Reverse(next_d), e.to));
            }
        }
    }
    (dist, prev)
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
