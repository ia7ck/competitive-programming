use scanner::Scanner;
use std::{collections::HashMap, io};

fn main() {
    let mut scanner = Scanner::from(io::stdin().lock());

    let (n, k, q) = scan!((usize, usize, usize), <~scanner);
    let mut a = scan!([[u64; k]; n], <~scanner);
    for i in 1..n {
        for j in 0..k {
            a[i][j] |= a[i - 1][j];
        }
    }
    for _ in 0..q {
        let m = scan!(usize, <~scanner);
        let mut lb = HashMap::<usize, u64>::new(); // exclusive
        let mut ub = HashMap::<usize, u64>::new(); // exclusive
        for _ in 0..m {
            let (j, op, x) = scan!((usize, char, u64), <~scanner);
            let j = j - 1;
            if op == '>' {
                // a[?][j] > x
                lb.entry(j).and_modify(|e| *e = (*e).max(x)).or_insert(x);
            } else {
                // a[?][j] < x
                ub.entry(j).and_modify(|e| *e = (*e).min(x)).or_insert(x);
            }
        }

        let satisfy_lb = |i: usize| -> bool { lb.iter().all(|(&j, &x)| a[i][j] > x) };
        let satisfy_ub = |i: usize| -> bool { ub.iter().all(|(&j, &x)| a[i][j] < x) };

        let mut ans = Option::<usize>::None;
        if satisfy_ub(0) {
            if satisfy_lb(0) {
                ans = Some(0);
            } else if satisfy_lb(n - 1) {
                let mut ok_ub = 0;
                let mut ng_ub = n;
                while ng_ub - ok_ub > 1 {
                    let mid = (ng_ub + ok_ub) / 2;
                    if satisfy_ub(mid) {
                        ok_ub = mid;
                    } else {
                        ng_ub = mid;
                    }
                }
                let mut ng_lb = 0;
                let mut ok_lb = n - 1;
                while ok_lb - ng_lb > 1 {
                    let mid = (ng_lb + ok_lb) / 2;
                    if satisfy_lb(mid) {
                        ok_lb = mid;
                    } else {
                        ng_lb = mid;
                    }
                }
                // ub: [true, true, ..., true, false, false, ..., false]
                // lb: [false, false, ..., false, true, true, ..., true]
                if ok_lb <= ok_ub {
                    ans = Some(ok_lb);
                }
            }
        }

        if let Some(ans) = ans {
            println!("{}", ans + 1);
        } else {
            println!("-1");
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
