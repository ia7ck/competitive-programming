fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn mpow(a: u64, x: u64, m: u64) -> u64 {
    match x {
        0 => 1,
        1 => a % m,
        x if x % 2 == 0 => mpow(a * a % m, x / 2, m),
        x if x % 2 == 1 => a * mpow(a, x - 1, m) % m,
        _ => unreachable!(),
    }
}

fn solve(a: u64, b: u64, c: u64) -> u64 {
    assert_eq!(gcd(a, 10), 1);
    let e = mpow(b, c, 4);
    mpow(a, e, 10)
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: u64 = rd.get();
    let b: u64 = rd.get();
    let c: u64 = rd.get();

    if a % 10 == 0 {
        println!("0");
        return;
    }
    if gcd(a, 10) == 1 {
        // 3 4 5
        println!("{}", solve(a, b, c));
        return;
    }
    if a % 2 == 0 {
        // 6 3 4
        // => 6
        let mut a = a;
        let mut n = 0;
        while a % 2 == 0 {
            a /= 2;
            n += 1;
        }
        assert!(n >= 1);
        assert_eq!(gcd(a, 10), 1);
        let x = match n * mpow(b, c, 4) % 4 {
            1 => 2,
            2 => 4,
            3 => 8,
            0 => 6,
            _ => unreachable!(),
        };
        let y = solve(a, b, c);
        println!("{}", x * y % 10);
    } else if a % 5 == 0 {
        // 15 4 3
        // => 5
        let mut a = a;
        while a % 5 == 0 {
            a /= 5;
        }
        assert_eq!(gcd(a, 10), 1);
        let ans = solve(a, b, c);
        println!("{}", ans * 5 % 10);
    } else {
        unreachable!()
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
