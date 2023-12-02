use scanner::Scanner;
use std::io;

fn main() {
    let mut scanner = Scanner::from(io::stdin().lock());
    let (a, b, c) = scan!((i32, i32, i32), <~ scanner);

    for d in 1..(b + c) {
        for e in 1..(b + c) {
            if d + a + e >= b + c || b >= c + (d + a + e) || c >= b + (d + a + e) {
                continue;
            }
            let beta = f64::acos(
                f64::from(b.pow(2) + (d + a + e).pow(2) - c.pow(2))
                    / f64::from(2 * b * (d + a + e)),
            );
            let gamma = f64::acos(
                f64::from(c.pow(2) + (d + a + e).pow(2) - b.pow(2))
                    / f64::from(2 * c * (d + a + e)),
            );
            let dd = f64::sqrt(f64::from(b.pow(2) + d.pow(2)) - f64::from(2 * b * d) * beta.cos());
            let ee = f64::sqrt(f64::from(c.pow(2) + e.pow(2)) - f64::from(2 * c * e) * gamma.cos());
            let v1 = (f64::from(b).powf(2.0) + dd.powf(2.0) - f64::from(d).powf(2.0))
                / (f64::from(2 * b) * dd);
            let v2 = (f64::from(c).powf(2.0) + ee.powf(2.0) - f64::from(e).powf(2.0))
                / (f64::from(2 * c) * ee);
            assert!(!v1.is_nan());
            assert!(!v2.is_nan());
            if (v1 - v2).abs() < 1e-6 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
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
