fn solve(n: usize, w: Vec<u64>, edges: Vec<(usize, usize)>) {
    use std::collections::BinaryHeap;
    let mut deg = vec![0; n];
    for (u, v) in edges {
        deg[u] += 1;
        deg[v] += 1;
    }
    let mut q = BinaryHeap::new();
    for i in 0..n {
        if deg[i] >= 2 {
            q.push((w[i], i));
        }
    }
    let mut ans = w.iter().sum::<u64>();
    print!("{}", ans);
    for _ in 1..(n - 1) {
        let (_, i) = q.pop().unwrap();
        ans += w[i];
        deg[i] -= 1;
        if deg[i] >= 2 {
            q.push((w[i], i));
        }
        print!(" {}", ans);
    }
    println!();
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let w: Vec<u64> = rd.get_vec(n);
        let edges: Vec<(usize, usize)> = (0..(n - 1))
            .map(|_| {
                let u: usize = rd.get();
                let v: usize = rd.get();
                (u - 1, v - 1)
            })
            .collect();
        solve(n, w, edges);
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
