fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let a: Vec<Vec<char>> = (0..h).map(|_| rd.get_chars()).collect();

    let a: Vec<Vec<i32>> = {
        let mut b = vec![vec![0; w]; h];
        for i in 0..h {
            for j in 0..w {
                if a[i][j] == '+' {
                    b[i][j] = 1;
                } else {
                    b[i][j] = -1;
                }
            }
        }
        b
    };

    // takahashi - aoki
    let mut dp = vec![vec![0; w]; h];
    for j in (0..(w - 1)).rev() {
        if (h - 1 + j) % 2 == 0 {
            dp[h - 1][j] = dp[h - 1][j + 1] + a[h - 1][j + 1];
        } else {
            dp[h - 1][j] = dp[h - 1][j + 1] + a[h - 1][j + 1] * (-1);
        }
    }
    for i in (0..(h - 1)).rev() {
        if (i + w - 1) % 2 == 0 {
            dp[i][w - 1] = dp[i + 1][w - 1] + a[i + 1][w - 1];
        } else {
            dp[i][w - 1] = dp[i + 1][w - 1] + a[i + 1][w - 1] * (-1);
        }
    }
    for i in (0..(h - 1)).rev() {
        for j in (0..(w - 1)).rev() {
            if (i + j) % 2 == 0 {
                dp[i][j] = (dp[i + 1][j] + a[i + 1][j]).max(dp[i][j + 1] + a[i][j + 1]);
            } else {
                dp[i][j] =
                    (dp[i + 1][j] + a[i + 1][j] * (-1)).min(dp[i][j + 1] + a[i][j + 1] * (-1));
            }
        }
    }
    // for row in &dp {
    //     eprintln!("{:?}", row);
    // }
    let diff = dp[0][0];
    if diff > 0 {
        println!("Takahashi");
    } else if diff < 0 {
        println!("Aoki");
    } else {
        println!("Draw");
    }
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
