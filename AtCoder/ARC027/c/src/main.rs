fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let x: usize = rd.get();
    let y: usize = rd.get();
    let n: usize = rd.get();
    let th: Vec<(usize, u64)> = (0..n)
        .map(|_| {
            let t: usize = rd.get();
            let h: u64 = rd.get();
            (t, h)
        })
        .collect();

    macro_rules! chmax {
        ($a: expr, $b: expr) => {
            $a = std::cmp::max($a, $b);
        };
    }

    let mut dp = vec![vec![0; x + y + 1]; x + 1];
    dp[x][x + y] = 0;
    for (t, h) in th {
        let mut nxt = dp.clone();
        for i in 1..=x {
            for j in t..=(x + y) {
                chmax!(nxt[i - 1][j - t], dp[i][j] + h);
            }
        }
        dp = nxt;
    }
    let mut ans = 0;
    for i in 0..dp.len() {
        for j in 0..dp[i].len() {
            chmax!(ans, dp[i][j]);
        }
    }
    println!("{}", ans);
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
