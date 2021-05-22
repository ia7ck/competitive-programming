fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let mut a: usize = rd.get();
    let mut b: usize = rd.get();
    let mut k: u64 = rd.get();

    let mut binom = vec![vec![0u64; a + b + 1]; a + b + 1];
    binom[0][0] = 1;
    binom[1][0] = 1;
    binom[1][1] = 1;
    for i in 2..=(a + b) {
        binom[i][0] = 1;
        for j in 1..=i {
            binom[i][j] = binom[i - 1][j - 1] + binom[i - 1][j];
        }
    }

    let mut ans = String::new();
    for _ in 0..(a + b) {
        if a == 0 {
            b -= 1;
            ans.push('b');
            continue;
        }
        if b == 0 {
            a -= 1;
            ans.push('a');
            continue;
        }
        if k > binom[a - 1 + b][b] {
            k -= binom[a - 1 + b][b];
            b -= 1;
            ans.push('b');
        } else {
            a -= 1;
            ans.push('a');
        }
        // println!("k={}", k);
    }
    assert_eq!(a, 0);
    assert_eq!(b, 0);
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
