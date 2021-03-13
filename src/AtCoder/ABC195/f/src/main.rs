fn is_prime(x: u64) -> bool {
    assert!(x >= 1);
    if x == 1 {
        return false;
    }
    (2..x).all(|y| x % y != 0)
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: u64 = rd.get();
    let b: u64 = rd.get();

    let primes: Vec<u64> = (2..=(b - a)).filter(|&x| is_prime(x)).collect();
    let mut dp = vec![0; 1 << primes.len()];
    dp[0] = 1;
    for x in a..=b {
        let mut nxt = dp.clone();
        let mut y = 0;
        for (i, &p) in primes.iter().enumerate() {
            if x % p == 0 {
                y ^= 1 << i;
            }
        }
        for bits in 0..(1 << primes.len()) {
            if bits & y != 0 {
                // x を選べない
                continue;
            }
            nxt[bits ^ y] += dp[bits];
        }
        dp = nxt;
    }
    let ans = dp.iter().sum::<u64>();
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
