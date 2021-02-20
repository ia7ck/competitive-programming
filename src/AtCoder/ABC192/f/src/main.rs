fn solve(x: usize, a: &[usize], m: usize) -> usize {
    let n = a.len();
    let mut dp = vec![vec![vec![None; m]; m + 1]; n + 1];
    dp[0][0][0] = Some(0);
    for i in 0..n {
        for j in 0..=m {
            for s in 0..m {
                if let Some(max) = dp[i][j][s] {
                    let y = dp[i + 1][j][s].get_or_insert(0);
                    *y = std::cmp::max(*y, max);
                    if j + 1 <= m {
                        let y = dp[i + 1][j + 1][(s + a[i]) % m].get_or_insert(0);
                        *y = std::cmp::max(*y, max + a[i]);
                    }
                }
            }
        }
    }
    if let Some(max) = dp[n][m][x % m] {
        assert!(x >= max);
        assert_eq!((x - max) % m, 0);
        return (x - max) / m;
    }
    std::usize::MAX
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let x: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let mut ans = std::usize::MAX;
    for m in 1..=n {
        let tmp = solve(x, &a, m);
        ans = ans.min(tmp);
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
