use scanner::Scanner;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::from(stdin.lock());

    let t = scan!(usize, <~ scanner);
    for _ in 0..t {
        let (n, k) = scan!((usize, u64), <~ scanner);
        let a = scan!([u64; n], <~ scanner);
        let b = scan!([u8; n], <~ scanner);
        solve(n, k, a, b);
    }
}

fn solve(n: usize, k: u64, a: Vec<u64>, b: Vec<u8>) {
    let mut ans = 0;

    // b = 1
    if let Some(i) = (0..n).filter(|&i| b[i] == 1).max_by_key(|&i| a[i]) {
        let mut c = Vec::new();
        for j in 0..n {
            if j != i {
                c.push(a[j]);
            }
        }
        c.sort();
        let med = c[(n - 2) / 2];
        ans = (a[i] + k) + med;
    }

    // b = 0
    if let Some(i) = (0..n).filter(|&i| b[i] == 0).max_by_key(|&i| a[i]) {
        let mut ab = Vec::new();
        for j in 0..n {
            if j != i {
                ab.push((a[j], b[j]));
            }
        }
        ab.sort();
        ab.reverse();

        // Median >= x
        // <=>
        // Count(>=x) > len/2
        let mut ok = 0;
        let mut ng = 1_000_000_000 + 1;
        while ng - ok > 1 {
            let m = (ok + ng) / 2;
            let mut count = 0;
            let mut op = 0;
            for &(a, b) in &ab {
                if a >= m {
                    count += 1;
                } else if b == 1 && op + (m - a) <= k {
                    count += 1;
                    op += m - a;
                }
            }
            if count > (n - 1) / 2 {
                ok = m;
            } else {
                ng = m;
            }
        }
        ans = ans.max(a[i] + ok);
    }
    println!("{}", ans);
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
