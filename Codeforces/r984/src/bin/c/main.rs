use scanner::Scanner;
use std::{collections::HashSet, io};

fn main() {
    let mut scanner = Scanner::from(io::stdin().lock());

    let t = scan!(usize, <~scanner);
    for _ in 0..t {
        let s = scan!(String, <~scanner);
        let q = scan!(usize, <~scanner);
        let queries = scan!([(usize, u8); q], <~scanner);
        solve(s.chars().collect(), queries);
    }
}

fn solve(s: Vec<char>, queries: Vec<(usize, u8)>) {
    let n = s.len();
    let mut pos1100 = HashSet::new();
    for i in 0..n {
        if i + 4 <= n && s[i..i + 4] == ['1', '1', '0', '0'] {
            pos1100.insert(i);
        }
    }

    let mut s = s;
    for (i, c) in queries {
        let i = i - 1;
        for j in i.saturating_sub(3)..=i {
            pos1100.remove(&j);
        }
        s[i] = char::from(c + b'0');
        for j in i.saturating_sub(3)..=i {
            if j + 4 <= n && s[j..j + 4] == ['1', '1', '0', '0'] {
                pos1100.insert(j);
            }
        }
        if pos1100.is_empty() {
            println!("NO");
        } else {
            println!("YES");
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
