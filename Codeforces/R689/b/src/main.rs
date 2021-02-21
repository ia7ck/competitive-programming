fn solve(n: usize, m: usize, a: &Vec<Vec<char>>) {
    let mut cum = vec![vec![0; m + 1]; n];
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == '*' {
                cum[i][j + 1] += 1;
            }
            cum[i][j + 1] += cum[i][j];
        }
    }
    let mut ans: u64 = 0;
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == '.' {
                continue;
            }
            for y in i..n {
                let d = y - i;
                if j >= d && j + d + 1 <= m {
                    let cnt = cum[y][j + d + 1] - cum[y][j - d];
                    if cnt == d * 2 + 1 {
                        ans += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let m: usize = rd.get();
        let a: Vec<Vec<char>> = (0..n)
            .map(|_| {
                let s: String = rd.get();
                s.chars().collect()
            })
            .collect();
        solve(n, m, &a);
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
