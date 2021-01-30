fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let k: usize = rd.get();
    let c: Vec<usize> = rd.get_vec(k);
    let c: Vec<usize> = c.iter().map(|&c| c - 1).collect();

    let inf = std::u64::MAX;
    let mut d = vec![vec![inf; k]; k];
    for i in 0..k {
        let dd = bfs(c[i], &g);
        for j in 0..k {
            d[i][j] = dd[c[j]];
        }
    }
    let mut dp = vec![vec![inf; k]; 1 << k];
    for i in 0..k {
        dp[1 << i][i] = 0;
    }
    macro_rules! chmin {
        ($a: expr, $b: expr) => {
            $a = std::cmp::min($a, $b);
        };
    }
    for bits in 0..(1 << k) {
        for last in 0..k {
            for nxt in 0..k {
                if dp[bits][last] < inf
                    && d[last][nxt] < inf
                    && (bits >> last & 1 == 1)
                    && (bits >> nxt & 1 == 0)
                {
                    chmin!(dp[bits ^ (1 << nxt)][nxt], dp[bits][last] + d[last][nxt]);
                }
            }
        }
    }
    let &ans = dp[(1 << k) - 1].iter().min().unwrap();
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans + 1);
    }
}

fn bfs(s: usize, g: &[Vec<usize>]) -> Vec<u64> {
    let inf = std::u64::MAX;
    let n = g.len();
    let mut d = vec![inf; n];
    d[s] = 0;
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back(s);
    while let Some(i) = q.pop_front() {
        for &j in &g[i] {
            if d[j] == inf {
                d[j] = d[i] + 1;
                q.push_back(j);
            }
        }
    }
    d
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
