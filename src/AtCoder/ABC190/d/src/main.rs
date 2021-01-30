fn divisors(x: i64) -> Vec<i64> {
    let mut res = vec![];
    for y in (1..).take_while(|y| y * y <= x) {
        if x % y == 0 {
            res.push(y);
            if x / y != y {
                res.push(x / y);
            }
        }
    }
    res
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: i64 = rd.get();

    let mut ans = 0;
    let divs = divisors(n * 2);
    for k in divs {
        let a2 = n * 2 / k - k + 1;
        if a2 % 2 == 0 {
            ans += 1;
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
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", rest));
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
                    self.l.clear();
                    self.i = 0;
                    let num_bytes = self
                        .r
                        .read_line(&mut self.l)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = self
                        .l
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
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
