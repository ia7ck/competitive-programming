fn dfs(i: usize, p: usize, g: &[Vec<usize>], par: &mut Vec<usize>) {
    par[i] = p;
    for &j in &g[i] {
        if j != p {
            dfs(j, i, g, par);
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let ab: Vec<(usize, usize)> = (0..(n - 1))
        .map(|_| {
            let a: usize = rd.get();
            let b: usize = rd.get();
            (a - 1, b - 1)
        })
        .collect();

    let mut g = vec![vec![]; n];
    for i in 0..(n - 1) {
        let (a, b) = ab[i];
        g[a].push(b);
        g[b].push(a);
    }
    let mut par = vec![!0; n];
    dfs(0, !0, &g, &mut par);

    let mut ans = vec![0; n];
    let q: usize = rd.get();
    for _ in 0..q {
        let t: usize = rd.get();
        match t {
            1 => {
                let i: usize = rd.get();
                let x: i64 = rd.get();
                let (a, b) = ab[i - 1];
                // a -> b
                if par[a] == b {
                    ans[a] += x;
                } else {
                    ans[0] += x;
                    ans[b] -= x;
                }
            }
            2 => {
                let i: usize = rd.get();
                let x: i64 = rd.get();
                let (a, b) = ab[i - 1];
                // b -> a
                if par[b] == a {
                    ans[b] += x;
                } else {
                    ans[0] += x;
                    ans[a] -= x;
                }
            }
            _ => unreachable!(),
        }
    }

    // println!("{:?}", down);
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back((0, !0));
    while let Some((i, p)) = q.pop_front() {
        for &j in &g[i] {
            if j != p {
                ans[j] += ans[i];
                q.push_back((j, i));
            }
        }
    }
    for ans in ans {
        println!("{}", ans);
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
