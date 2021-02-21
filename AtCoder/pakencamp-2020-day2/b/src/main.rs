fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let a: Vec<Vec<char>> = (0..h)
        .map(|_| rd.get::<String>().chars().collect())
        .collect();

    macro_rules! chmin {
        ($a: expr, $b: expr) => {
            $a = std::cmp::min($a, $b);
        };
    }

    let inf = std::u32::MAX;
    let mut dp = vec![vec![inf; w + 1]; h + 1];
    dp[0][0] = 0;
    for i in 0..h {
        for j in 0..w {
            match a[i][j] {
                'E' => {
                    chmin!(dp[i][j + 1], dp[i][j]);
                    chmin!(dp[i + 1][j], dp[i][j] + 1);
                }
                'S' => {
                    chmin!(dp[i + 1][j], dp[i][j]);
                    chmin!(dp[i][j + 1], dp[i][j] + 1);
                }
                _ => unreachable!(),
            }
        }
    }
    println!("{}", dp[h - 1][w]);
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
