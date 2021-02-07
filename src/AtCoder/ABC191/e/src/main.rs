fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let mut g = vec![vec![]; n * 2];
    let id = |v: usize, moved: bool| -> usize { v + n * moved as usize };
    // 0, 1, ..., n-1
    // n, n+1, ..., 2n-1
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        let c: u64 = rd.get();
        let a = a - 1;
        let b = b - 1;
        g[id(a, false)].push(Edge {
            to: id(b, true),
            cost: c,
        });
        g[id(a, true)].push(Edge {
            to: id(b, true),
            cost: c,
        });
    }
    for s in 0..n {
        let (d, _) = dijkstra(&g, s);
        let ans = d[id(s, true)];
        if ans == std::u64::MAX {
            println!("{}", -1);
        } else {
            println!("{}", ans);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub to: usize,
    pub cost: u64,
}

#[allow(clippy::many_single_char_names)]
pub fn dijkstra(g: &[Vec<Edge>], s: usize) -> (Vec<u64>, Vec<Option<usize>>) {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = g.len();
    let mut d = vec![std::u64::MAX; n];
    let mut q = BinaryHeap::new();
    let mut prev = vec![None; n];
    d[s] = 0;
    q.push((Reverse(0), s));
    while let Some((Reverse(c), v)) = q.pop() {
        if c > d[v] {
            continue;
        }
        for e in &g[v] {
            if c + e.cost < d[e.to] {
                d[e.to] = c + e.cost;
                prev[e.to] = Some(v);
                q.push((Reverse(d[e.to]), e.to));
            }
        }
    }
    (d, prev)
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
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", rest));
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
                    self.l.clear();
                    self.i = 0;
                    let num_bytes = self
                        .r
                        .read_line(&mut self.l)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = self
                        .l
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
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
