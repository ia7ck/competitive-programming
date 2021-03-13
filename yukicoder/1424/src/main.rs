fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut g = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let a: usize = rd.get();
        let b: usize = rd.get();
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let max = max_dist(&g);
    let min = min_dist(&g);
    if max == min {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn max_dist(g: &Vec<Vec<usize>>) -> usize {
    macro_rules! chmax {
        ($a: expr, $b: expr) => {
            $a = std::cmp::max($a, $b);
        };
    }
    fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, par: &mut Vec<usize>, d: &mut Vec<usize>) {
        par[i] = p;
        d[i] = 0;
        for &j in &g[i] {
            if j == p {
                continue;
            }
            dfs(j, i, g, par, d);
            chmax!(d[i], d[j] + 1);
        }
    }
    let n = g.len();
    let mut par = vec![0; n];
    let mut sub_d = vec![0; n];
    dfs(0, std::usize::MAX, &g, &mut par, &mut sub_d);
    let mut d = vec![0; n];
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back((0, std::usize::MAX, 0));
    while let Some((u, p, par_d)) = q.pop_front() {
        d[u] = par_d.max(sub_d[u]);
        let k = g[u].len();
        let mut head = vec![0; k + 1];
        for (i, &v) in g[u].iter().enumerate() {
            chmax!(
                head[i + 1],
                head[i].max(if v == p { par_d } else { sub_d[v] + 1 })
            );
        }
        let mut tail = vec![0; k + 1];
        for (i, &v) in g[u].iter().enumerate().rev() {
            chmax!(
                tail[i],
                tail[i + 1].max(if v == p { par_d } else { sub_d[v] + 1 })
            )
        }
        for (i, &v) in g[u].iter().enumerate() {
            if v != p {
                q.push_back((v, u, 1 + head[i].max(tail[i + 1])));
            }
        }
    }
    d.iter().copied().max().unwrap()
}

fn min_dist(g: &Vec<Vec<usize>>) -> usize {
    let n = g.len();
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    let mut d = vec![std::usize::MAX; n];
    for i in 0..n {
        if g[i].len() == 1 {
            d[i] = 0;
            q.push_back((i, std::usize::MAX));
        }
    }
    while let Some((u, prev)) = q.pop_front() {
        for &v in &g[u] {
            if v != prev {
                if d[v] != std::usize::MAX {
                    return d[v] + d[u] + 1;
                } else {
                    d[v] = d[u] + 1;
                    q.push_back((v, u));
                }
            }
        }
    }
    unreachable!()
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
