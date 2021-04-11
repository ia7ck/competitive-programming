fn solve(a: usize, b: usize, s: Vec<char>) {
    let n = s.len();
    let mut s = s;
    for i in 0..(n / 2) {
        match (s[i], s[n - i - 1]) {
            ('?', '?') => {}
            ('?', c) => {
                s[i] = c;
            }
            (c, '?') => {
                s[n - i - 1] = c;
            }
            (c, d) => {
                if c != d {
                    println!("-1");
                    return;
                }
            }
        }
    }
    let mut zero = s.iter().filter(|&&c| c == '0').count();
    let mut one = s.iter().filter(|&&c| c == '1').count();
    if zero > a || one > b {
        println!("-1");
        return;
    }
    for i in 0..(n / 2) {
        if s[i] == '?' {
            assert_eq!(s[n - i - 1], '?');
            if zero + 2 <= a {
                zero += 2;
                s[i] = '0';
                s[n - i - 1] = '0';
            } else if one + 2 <= b {
                one += 2;
                s[i] = '1';
                s[n - i - 1] = '1';
            } else {
                println!("-1");
                return;
            }
        }
    }
    if n % 2 == 1 && s[n / 2] == '?' {
        if zero < a {
            zero += 1;
            s[n / 2] = '0';
        } else {
            one += 1;
            s[n / 2] = '1';
        }
    }
    assert_eq!(zero, a);
    assert_eq!(one, b);
    for c in s {
        print!("{}", c);
    }
    println!();
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let a: usize = rd.get();
        let b: usize = rd.get();
        let s: Vec<char> = rd.get_chars();
        solve(a, b, s);
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
