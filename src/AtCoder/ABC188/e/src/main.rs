fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let a: Vec<i64> = rd.get_vec(n);
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let x: usize = rd.get();
        let y: usize = rd.get();
        g[x - 1].push(y - 1);
    }

    let mut in_deg = vec![0; n];
    for i in 0..n {
        for &j in &g[i] {
            in_deg[j] += 1;
        }
    }
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    let mut ord = vec![];
    for i in 0..n {
        if in_deg[i] == 0 {
            q.push_back(i);
            ord.push(i);
        }
    }
    while let Some(i) = q.pop_front() {
        for &j in &g[i] {
            in_deg[j] -= 1;
            if in_deg[j] == 0 {
                q.push_back(j);
                ord.push(j);
            }
        }
    }
    // println!("{:?}", ord);
    let inf = std::i64::MAX / 2;
    let mut mins = vec![inf; n];
    for i in ord { // [0, 1, 2, 3, ...]
        for &j in &g[i] {
            mins[j] = mins[j].min(mins[i]).min(a[i]);
        }
    }
    // println!("{:?}", mins);
    assert!(mins.iter().any(|&x| x != inf));
    let mut ans = std::i64::MIN;
    for i in 0..n {
        ans = ans.max(a[i] - mins[i]);
    }
    println!("{}", ans);
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
