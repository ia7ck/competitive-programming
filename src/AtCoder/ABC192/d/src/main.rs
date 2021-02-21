fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let x: String = rd.get();
    let x: Vec<u64> = x.bytes().map(|b| (b - b'0') as u64).collect();
    let m: u64 = rd.get();

    if x.len() == 1 {
        if x[0] <= m {
            println!("1");
        } else {
            println!("0");
        }
        return;
    }
    let mut x = x;
    x.reverse();
    let le_m = |base: u64| {
        let mut y = 0;
        for (i, &x) in x.iter().enumerate() {
            // y = base^i * x[i] + y
            let nxt_y = base
                .checked_pow(i as u32)
                .and_then(|a| a.checked_mul(x))
                .and_then(|a| a.checked_add(y));
            match nxt_y {
                Some(nxt_y) => {
                    y = nxt_y;
                }
                None => {
                    return false;
                }
            }
        }
        y <= m
    };

    let d = x.iter().copied().max().unwrap();
    if !le_m(d + 1) {
        println!("0");
        return;
    }
    let mut ok = d + 1;
    let mut ng = std::u64::MAX / 2;
    while ng - ok > 1 {
        let base = (ng + ok) / 2;
        if le_m(base) {
            ok = base;
        } else {
            ng = base;
        }
    }
    println!("{}", ok - d);
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
