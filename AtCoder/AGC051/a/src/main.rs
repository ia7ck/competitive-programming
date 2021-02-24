fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let d: usize = rd.get();
    let mo = 998_244_353;
    let (fac, fac_inv) = factorials(d * 2 + 1, mo);
    let binom = |n, k| {
        if n < k {
            0
        } else {
            fac[n] * fac_inv[k] % mo * fac_inv[n - k] % mo
        }
    };
    let ans = binom(d * 2 - 1, d);
    println!("{}", ans);
}

pub fn factorials(size: usize, mo: u64) -> (Vec<u64>, Vec<u64>) {
    let mut fac = vec![0; size];
    let mut inv = vec![0; size];
    let mut inv_fac = vec![0; size];
    fac[0] = 1;
    fac[1] = 1;
    inv[1] = 1;
    inv_fac[0] = 1;
    inv_fac[1] = 1;
    for i in 2..size {
        let i_u64 = i as u64;
        fac[i] = fac[i - 1] * i_u64 % mo;
        inv[i] = ((mo - inv[(mo as usize) % i]) * (mo / i_u64)).rem_euclid(mo);
        inv_fac[i] = inv_fac[i - 1] * inv[i] % mo;
    }
    (fac, inv_fac)
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
