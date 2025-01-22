use scanner::Scanner;

use std::io;

fn main() {
    let mut scanner = Scanner::from(io::stdin().lock());

    let t = scan!(usize, <~scanner);
    for _ in 0..t {
        let (n, l, r) = scan!((usize, usize, usize), <~scanner);
        let a = scan!([u64; n], <~scanner);
        solve(n, l, r, a);
    }
}

fn solve(_n: usize, l: usize, r: usize, a: Vec<u64>) {
    let mut left = a[..(l - 1)].to_vec();
    let mut middle = a[(l - 1)..r].to_vec();
    let mut right = a[r..].to_vec();

    left.sort_unstable();
    middle.sort_unstable();
    middle.reverse();
    right.sort_unstable();

    let mut ans1 = middle.iter().sum::<u64>();
    for (&left, &middle) in left.iter().zip(&middle) {
        if left < middle {
            ans1 -= middle - left;
        } else {
            break;
        }
    }
    let mut ans2 = middle.iter().sum::<u64>();
    for (&right, &middle) in right.iter().zip(&middle) {
        if right < middle {
            ans2 -= middle - right;
        } else {
            break;
        }
    }

    println!("{}", ans1.min(ans2));
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
