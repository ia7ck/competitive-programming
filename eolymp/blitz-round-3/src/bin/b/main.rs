use scanner::Scanner;

fn main() {
    let mut scanner = Scanner::stdin_lock();

    scan! {
        via scanner,
        t: usize,
    };

    for _ in 0..t {
        scan! {
            via scanner,
            n: usize,
            b: Usize1,
            s: [u64; n],
            t: [u64; n],
        };

        solve(n, b, s, t);
    }
}

fn solve(n: usize, b: usize, s: Vec<u64>, t: Vec<u64>) {
    let mut ord = (0..n).collect::<Vec<_>>();
    ord.sort_by_key(|&i| dist(s[i], t[i]));
    ord.reverse();
    let p = ord.partition_point(|&i| dist(s[i], t[i]) > dist(s[b], t[b]));
    println!("{}", p + 1);
}

fn dist(s: u64, t: u64) -> u64 {
    // s + (s - t) + (s - t * 2) + ...

    let k = (s + t - 1) / t;
    k * (s * 2 - (k - 1) * t) / 2
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod scanner {
    use std::{
        fmt,
        io::{self, BufReader, Cursor},
        str,
    };

    pub struct Scanner<R>
    where
        R: io::BufRead,
    {
        reader: R,
        buf: String,
        pos: usize,
    }

    impl Scanner<BufReader<io::StdinLock<'static>>> {
        pub fn stdin_lock() -> Self {
            Self {
                reader: BufReader::new(io::stdin().lock()),
                buf: String::new(),
                pos: 0,
            }
        }
    }

    impl<T> Scanner<Cursor<T>>
    where
        T: AsRef<[u8]>,
    {
        pub fn cursor(inner: T) -> Self {
            Self {
                reader: Cursor::new(inner),
                buf: String::new(),
                pos: 0,
            }
        }
    }

    impl<R> Scanner<R>
    where
        R: io::BufRead,
    {
        pub fn scan<T>(&mut self) -> T
        where
            T: str::FromStr,
            T::Err: fmt::Debug,
        {
            loop {
                match self.buf[self.pos..].find(|ch| !char::is_ascii_whitespace(&ch)) {
                    Some(j) => {
                        self.pos += j;
                        break;
                    }
                    None => {
                        let num_bytes = self
                            .reader
                            .read_line(&mut self.buf)
                            .unwrap_or_else(|_| panic!("invalid UTF-8"));
                        assert!(num_bytes > 0, "reached EOF :(");
                    }
                }
            }

            let rest = &self.buf[self.pos..];
            let token_len = rest
                .find(|ch| char::is_ascii_whitespace(&ch))
                .unwrap_or(rest.len());
            let value = rest[..token_len]
                .parse()
                .unwrap_or_else(|e| panic!("{:?}, attempt to read `{}`", e, rest));
            self.pos += token_len;

            value
        }
    }

    #[macro_export]
    macro_rules! scan {
        (via $scanner:expr, $($rest:tt)*) => {
            scan!(@via [$scanner] @rest $($rest)*);
        };

        (@via [$via:expr] @rest) => {};
        (@via [$via:expr] @rest ,) => {};

        (@via [$via:expr] @rest mut $($rest:tt)*) => {
            $crate::scan!(@via [$via] @mut [mut] @rest $($rest)*);
        };
        (@via [$via:expr] @rest $($rest:tt)*) => {
            scan!(@via [$via] @mut [] @rest $($rest)*);
        };

        (@via [$via:expr] @mut [$($mut:tt)?] @rest $var:tt : $t:tt) => {
            let $($mut)? $var = scan_inner!(via $via, $t);
        };
        (@via [$via:expr] @mut [$($mut:tt)?] @rest $var:tt : $t:tt , $($rest:tt)*) => {
            scan!(@via [$via] @mut [$($mut)?] @rest $var : $t);
            scan!(@via [$via] @rest $($rest)*);
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! scan_inner {
        (via $scanner:expr, ( $($t:tt),* )) => {
            ( $($crate::scan_inner!(via $scanner, $t)),* )
        };

        (via $scanner:expr, [ $t:tt ; $len:expr ]) => {
            ::std::iter::repeat_with(|| scan_inner!(via $scanner, $t)).take($len).collect::<Vec<_>>()
        };

        (via $scanner:expr, Usize1) => {
            $scanner.scan::<usize>().checked_sub(1).expect("Usize1: input was 0, expected >= 1")
        };

        (via $scanner:expr, Chars) => {
            $scanner.scan::<String>().chars().collect::<Vec<_>>()
        };

        (via $scanner:expr, Bytes) => {
            $scanner.scan::<String>().bytes().collect::<Vec<_>>()
        };

        (via $scanner:expr, $ty:ty) => {
            $scanner.scan::<$ty>()
        };
    }
}
