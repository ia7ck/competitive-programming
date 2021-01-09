type E = (usize, u64);

fn dfs(i: usize, p: usize, g: &Vec<Vec<E>>, size: &mut Vec<usize>, dp: &mut Vec<u64>) {
    size[i] = 1;
    for &(j, w) in &g[i] {
        if j == p {
            continue;
        }
        dfs(j, i, g, size, dp);
        size[i] += size[j];
        dp[i] += dp[j] + w * size[j] as u64;
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut g = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let u: usize = rd.get();
        let v: usize = rd.get();
        let w: u64 = rd.get();
        g[u - 1].push((v - 1, w));
        g[v - 1].push((u - 1, w));
    }
    let mut size = vec![0; n];
    let mut dp = vec![0; n];
    dfs(0, !0, &g, &mut size, &mut dp);
    let mut ans = vec![0; n];
    ans[0] = dp[0];
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back((0, !0));
    while let Some((i, p)) = q.pop_front() {
        for &(j, w) in &g[i] {
            if j == p {
                continue;
            }
            ans[j] = ans[i] - size[j] as u64 * w + (n - size[j]) as u64 * w;
            q.push_back((j, i));
        }
    }
    println!("{}", ans.iter().sum::<u64>());
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
