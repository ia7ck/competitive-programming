fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let s: usize = rd.get();
    let goal: usize = rd.get();
    let s = s - 1;
    let goal = goal - 1;
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        let t: u64 = rd.get();
        let k: u64 = rd.get();
        g[a - 1].push(Edge {
            to: b - 1,
            cost: t,
            k,
        });
        g[b - 1].push(Edge {
            to: a - 1,
            cost: t,
            k,
        });
    }
    let (d, _prev) = dijkstra(&g, s);
    if let Some(ans) = d[goal] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub to: usize,
    pub cost: u64,
    pub k: u64,
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
            let next_d = (d + e.k - 1) / e.k * e.k + e.cost;
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
