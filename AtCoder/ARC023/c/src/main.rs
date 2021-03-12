fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<Option<usize>> = (0..n)
        .map(|_| {
            let x: isize = rd.get();
            if x != -1 {
                Some(x as usize)
            } else {
                None
            }
        })
        .collect();

    let ai: Vec<(usize, usize)> = a
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, x)| (i, x.unwrap()))
        .collect();

    let mo = 1_000_000_000 + 7;
    let (_, inv_fac) = factorials(n + 1, mo);
    // O(y) time
    let binom = |x: usize, y: usize| -> u64 {
        if x < y {
            return 0;
        }
        // x * (x-1) * ... * (x-y+1) / y / (y-1) / ... / 2 / 1
        let numer = ((x - y + 1)..=x).fold(1, |acc, z| acc * (z as u64) % mo);
        numer * inv_fac[y] % mo
    };
    let mut ans = 1;
    for w in ai.windows(2) {
        let (i, a) = w[0];
        let (j, b) = w[1];
        ans *= binom((b - a) + (j - i - 1), j - i - 1);
        ans %= mo;
    }
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
