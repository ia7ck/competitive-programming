use scanner::Scanner;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::from(stdin.lock());

    let t = scan!(usize, <~scanner);

    for _ in 0..t {
        let (n, d, k) = scan!((usize, usize, usize), <~scanner);
        let lr = scan!([(usize, usize); k], <~scanner);

        solve(n, d, k, lr);
    }
}

fn solve(n: usize, d: usize, _k: usize, lr: Vec<(usize, usize)>) {
    let mut start = vec![0; n];
    let mut end = vec![0; n];
    for &(l, r) in &lr {
        start[l - 1] += 1;
        end[r - 1] += 1;
    }
    let mut v = 0;
    for i in 0..d {
        v += start[i];
    }
    let mut day_bros = 0;
    let mut val_bros = v;
    let mut day_moth = 0;
    let mut val_moth = v;
    for i in d..n {
        assert!(v >= end[i - d]);
        v -= end[i - d];
        v += start[i];
        if val_bros < v {
            val_bros = v;
            day_bros = i - d + 1;
        }
        if val_moth > v {
            val_moth = v;
            day_moth = i - d + 1;
        }
    }
    println!("{} {}", day_bros + 1, day_moth + 1);
}

// ✂ --- scanner --- ✂
#[allow(unused)]
mod scanner {
    use std::fmt;
    use std::io;
    use std::str;

    pub struct Scanner<R> {
        r: R,
        l: String,
        i: usize,
    }

    impl<R> Scanner<R>
    where
        R: io::BufRead,
    {
        pub fn new(reader: R) -> Self {
            Self {
                r: reader,
                l: String::new(),
                i: 0,
            }
        }

        pub fn scan<T>(&mut self) -> T
        where
            T: str::FromStr,
            T::Err: fmt::Debug,
        {
            self.skip_blanks();
            assert!(self.i < self.l.len()); // remain some character
            assert_ne!(&self.l[self.i..=self.i], " ");
            let rest = &self.l[self.i..];
            let len = rest
                .find(|ch| char::is_ascii_whitespace(&ch))
                .unwrap_or_else(|| rest.len());
            // parse self.l[self.i..(self.i + len)]
            let val = rest[..len]
                .parse()
                .unwrap_or_else(|e| panic!("{:?}, attempt to read `{}`", e, rest));
            self.i += len;
            val
        }

        pub fn scan_vec<T>(&mut self, n: usize) -> Vec<T>
        where
            T: str::FromStr,
            T::Err: fmt::Debug,
        {
            (0..n).map(|_| self.scan()).collect::<Vec<_>>()
        }

        fn skip_blanks(&mut self) {
            loop {
                match self.l[self.i..].find(|ch| !char::is_ascii_whitespace(&ch)) {
                    Some(j) => {
                        self.i += j;
                        break;
                    }
                    None => {
                        self.l.clear(); // clear buffer
                        let num_bytes = self
                            .r
                            .read_line(&mut self.l)
                            .unwrap_or_else(|_| panic!("invalid UTF-8"));
                        assert!(num_bytes > 0, "reached EOF :(");
                        self.i = 0;
                    }
                }
            }
        }
    }

    impl<'a> From<&'a str> for Scanner<&'a [u8]> {
        fn from(s: &'a str) -> Self {
            Self::new(s.as_bytes())
        }
    }

    impl<'a> From<io::StdinLock<'a>> for Scanner<io::BufReader<io::StdinLock<'a>>> {
        fn from(stdin: io::StdinLock<'a>) -> Self {
            Self::new(io::BufReader::new(stdin))
        }
    }

    #[macro_export]
    macro_rules! scan {
        (( $($t: ty),+ ), <~ $scanner: expr) => {
            ( $(scan!($t, <~ $scanner)),+ )
        };
        ([ $t: tt; $n: expr ], <~ $scanner: expr) => {
            (0..$n).map(|_| scan!($t, <~ $scanner)).collect::<Vec<_>>()
        };
        ($t: ty, <~ $scanner: expr) => {
            $scanner.scan::<$t>()
        };
    }
}
// ✂ --- scanner --- ✂
