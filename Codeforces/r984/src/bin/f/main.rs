use scanner::Scanner;
use std::io;

fn main() {
    let mut scanner = Scanner::from(io::stdin().lock());

    let t = scan!(usize, <~scanner);
    for _ in 0..t {
        let (l, r, i, k) = scan!((u64, u64, u32, u64), <~scanner);
        solve(l, r, i, k);
    }
}

fn solve(l: u64, r: u64, i: u32, k: u64) {
    let ans = p_solve(r, i, k) ^ p_solve(l - 1, i, k);
    println!("{}", ans);
}

fn p_solve(r: u64, i: u32, k: u64) -> u64 {
    let mut ans = p_xor(r);
    let c = p_count(r, i, k);
    if r >= k {
        let y = match c % 4 {
            0 => 0,
            1 => (c - 1) * (1 << i) + k,
            2 => 1 << i,
            3 => c * (1 << i) + k,
            _ => unreachable!(),
        };
        ans ^= y;
    }
    ans
}

// 0 xor 1 xor ... xor r
fn p_xor(r: u64) -> u64 {
    match r % 4 {
        0 => r,
        1 => 1,
        2 => r + 1,
        3 => 0,
        _ => unreachable!(),
    }
}

// x <= r, x % (2^i) = k
fn p_count(r: u64, i: u32, k: u64) -> u64 {
    if r < k {
        return 0;
    }

    1 + (r - k) / (1 << i)
}

#[cfg(test)]
mod tests {
    use super::{p_count, p_solve, p_xor};

    #[test]
    fn test_solve() {
        fn naive(l: u64, r: u64, i: u32, k: u64) -> u64 {
            (l..=r).fold(0, |acc, x| if x % (1 << i) == k { acc } else { acc ^ x })
        }

        for l in 1..100 {
            for r in l..100 {
                for i in 0..6 {
                    for k in 0..(1 << i) {
                        assert_eq!(p_solve(r, i, k) ^ p_solve(l - 1, i, k), naive(l, r, i, k));
                    }
                }
            }
        }
    }

    #[test]
    fn test_p_count() {
        fn naive(r: u64, i: u32, k: u64) -> u64 {
            (k..=r).filter(|&x| x % (1 << i) == k).count() as u64
        }

        for r in 0..100 {
            for i in 0..6 {
                for k in 0..(1 << i) {
                    assert_eq!(p_count(r, i, k), naive(r, i, k));
                }
            }
        }
    }

    #[test]
    fn test_p_xor() {
        fn naive(r: u64) -> u64 {
            (0..=r).fold(0, |acc, x| acc ^ x)
        }

        for r in 0..100 {
            assert_eq!(p_xor(r), naive(r));
        }
    }

    #[test]
    fn test_p_solve() {
        fn naive(r: u64, i: u32, k: u64) -> u64 {
            (0..=r).fold(0, |acc, x| if x % (1 << i) == k { acc } else { acc ^ x })
        }

        for r in 0..100 {
            for i in 0..6 {
                for k in 0..(1 << i) {
                    assert_eq!(p_solve(r, i, k), naive(r, i, k));
                }
            }
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
