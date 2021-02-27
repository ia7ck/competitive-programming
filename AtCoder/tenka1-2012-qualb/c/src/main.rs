fn time(s: String) -> usize {
    // hh:mm
    let h = s[..2].parse::<usize>().unwrap();
    let m = s[3..].parse::<usize>().unwrap();
    h * 60 + m
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let st: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let s: String = rd.get();
            let t: String = rd.get();
            (time(s), time(t))
        })
        .collect();

    let mut dp = vec![0; 1 << n];
    for bits in 0..(1 << n) {
        dp[bits] = bits.count_ones();
    }
    for bits in 0..(1 << n) {
        let mut ok = true;
        for i in 0..n {
            for j in 0..i {
                if (bits >> i & 1 == 1) && (bits >> j & 1 == 1) {
                    let (s, t) = st[i];
                    let (u, v) = st[j];
                    let ((s, t), (u, v)) = if (s, t) <= (u, v) {
                        ((s, t), (u, v))
                    } else {
                        ((u, v), (s, t))
                    };
                    if !(t <= u && v <= s + 24 * 60) {
                        ok = false;
                    }
                }
            }
        }
        if ok {
            dp[bits] = 1;
        }
        let mut sub = bits;
        loop {
            dp[bits] = dp[bits].min(dp[sub] + dp[bits ^ sub]);
            if sub == 0 {
                break;
            }
            sub = (sub - 1) & bits;
        }
    }
    println!("{}", dp[(1 << n) - 1]);
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
