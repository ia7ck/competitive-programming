fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut s: Vec<char> = rd.get_chars();

    let mut flip = false;
    let q: usize = rd.get();
    for _ in 0..q {
        let t: usize = rd.get();
        let a: usize = rd.get();
        let b: usize = rd.get();
        match t {
            1 => {
                let a = a - 1;
                let b = b - 1;
                if flip {
                    s.swap((a + n) % (n * 2), (b + n) % (n * 2));
                } else {
                    s.swap(a, b);
                }
            }
            2 => {
                flip = !flip;
            }
            _ => unreachable!(),
        }
    }
    let mut ans = String::new();
    for i in 0..(n * 2) {
        if flip {
            ans.push(s[(i + n) % (n * 2)]);
        } else {
            ans.push(s[i]);
        }
    }
    println!("{}", ans);
}

// FLIP -> ILFP -> FPIL -> PFIL -> PIFL -> FLPI -> ILPF

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
