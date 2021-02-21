#[derive(Copy, Clone, Debug)]
struct Edge {
    to: usize,
    cost: u64,
}

fn dijkstra(g: &Vec<Vec<Edge>>, start: usize) -> (Vec<u64>, Vec<Option<usize>>) {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = g.len();
    let mut d = vec![std::u64::MAX; n];
    let mut q = BinaryHeap::new();
    let mut prev = vec![None; n];
    d[start] = 0;
    q.push((Reverse(0), start));
    while let Some((Reverse(c), cur)) = q.pop() {
        if c > d[cur] {
            continue;
        }
        for e in &g[cur] {
            if c + e.cost < d[e.to] {
                d[e.to] = c + e.cost;
                prev[e.to] = Some(cur);
                q.push((Reverse(d[e.to]), e.to));
            }
        }
    }
    (d, prev)
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    let n: usize = rd.get();
    let m: usize = rd.get();
    let edges: Vec<(usize, usize, u64)> = (0..m)
        .map(|_| {
            let u: usize = rd.get();
            let v: usize = rd.get();
            let w: u64 = rd.get();
            (u - 1, v - 1, w)
        })
        .collect();

    let solve0 = || {
        let mut ans = std::u64::MAX;
        for i in 0..m {
            let mut g = vec![vec![]; n];
            for j in 0..m {
                if i == j {
                    continue;
                }
                let (u, v, w) = edges[j];
                g[u].push(Edge { to: v, cost: w });
                g[v].push(Edge { to: u, cost: w });
            }
            let (u, v, w) = edges[i];
            let (d, _) = dijkstra(&g, v);
            ans = ans.min(d[u].saturating_add(w));
        }
        ans
    };

    let solve1 = || {
        let mut ans = std::u64::MAX;
        for i in 0..m {
            let mut g = vec![vec![]; n];
            for j in 0..m {
                if i == j {
                    continue;
                }
                let (u, v, w) = edges[j];
                g[u].push(Edge { to: v, cost: w });
            }
            let (u, v, w) = edges[i];
            let (d, _) = dijkstra(&g, v);
            ans = ans.min(d[u].saturating_add(w));
        }
        ans
    };

    let ans = match t {
        0 => solve0(),
        1 => solve1(),
        _ => unreachable!(),
    };
    if ans < std::u64::MAX {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }
}

pub struct ProconReader<R> {
    r: R,
    line: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            line: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.line.len());
        assert_ne!(&self.line[self.i..=self.i], " ");
        let line = &self.line[self.i..];
        let end = line.find(' ').unwrap_or_else(|| line.len());
        let s = &line[..end];
        self.i += end;
        s.parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", self.line))
    }
    fn skip_blanks(&mut self) {
        loop {
            let start = self.line[self.i..].find(|ch| ch != ' ');
            match start {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    self.line.clear();
                    self.i = 0;
                    let num_bytes = self.r.read_line(&mut self.line).unwrap();
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.line = self.line.trim_end_matches(&['\r', '\n'][..]).to_string();
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
}
