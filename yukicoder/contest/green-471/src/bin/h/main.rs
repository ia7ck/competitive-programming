use scanner::Scanner;
use std::io;

fn check((x1, y1, d1): (i64, i64, char), (x2, y2, d2): (i64, i64, char)) -> bool {
    match (d1, d2) {
        ('R', 'L') => y1 == y2 && x1 < x2,
        ('R', 'U') => x1 < x2 && y1 > y2 && x1.abs_diff(x2) == y1.abs_diff(y2), // (x2, y1)
        ('R', 'D') => x1 < x2 && y1 < y2 && x1.abs_diff(x2) == y1.abs_diff(y2),
        ('L', 'U') => x1 > x2 && y1 > y2 && x1.abs_diff(x2) == y1.abs_diff(y2),
        ('L', 'D') => x1 > x2 && y1 < y2 && x1.abs_diff(x2) == y1.abs_diff(y2),
        ('U', 'D') => x1 == x2 && y1 < y2,
        ('L', 'R') | ('U', 'R') | ('U', 'L') | ('D', 'R') | ('D', 'L') | ('D', 'U') => {
            // delegate
            check((x2, y2, d2), (x1, y1, d1))
        }
        _ => false,
    }
}

fn main() {
    let mut scanner = Scanner::from(io::stdin().lock());
    let t = scan!(usize, <~ scanner);
    for _ in 0..t {
        let (x1, y1, d1) = scan!((i64, i64, char), <~ scanner);
        let (x2, y2, d2) = scan!((i64, i64, char), <~ scanner);
        if check((x1, y1, d1), (x2, y2, d2)) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
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
