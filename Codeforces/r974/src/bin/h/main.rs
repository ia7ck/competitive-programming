use scanner::Scanner;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::from(stdin.lock());

    let t = scan!(usize, <~scanner);

    for _ in 0..t {
        let (n, q) = scan!((usize, usize), <~scanner);
        let a = scan!([usize; n], <~scanner);
        let queries = scan!([(usize, usize); q], <~scanner);
        let queries = queries
            .into_iter()
            .map(|(l, r)| (l - 1, r - 1))
            .collect::<Vec<_>>();

        solve(n, q, a, queries);
    }
}

fn solve(n: usize, q: usize, a: Vec<usize>, queries: Vec<(usize, usize)>) {
    let sqrt_q = (q as f64).sqrt() as usize;
    let mut ord = (0..q).collect::<Vec<_>>();
    ord.sort_by_key(|&i| {
        let (l, r) = queries[i];
        (l * sqrt_q / n, r)
    });

    struct S {
        present_odd: Vec<bool>,
        count: usize,
    }

    let add = |state: &mut S, i: usize| {
        if state.present_odd[a[i]] {
            state.present_odd[a[i]] = false;
            state.count -= 1;
        } else {
            state.present_odd[a[i]] = true;
            state.count += 1;
        }
    };

    let remove = |state: &mut S, i: usize| {
        // ok
        add(state, i);
    };

    let mut state = S {
        present_odd: vec![false; 1_000_000 + 1],
        count: 0,
    };
    state.present_odd[a[0]] = true;
    state.count += 1;
    let (mut left, mut right) = (0, 0);
    let mut ans = vec![false; q];
    for i in ord {
        let (l, r) = queries[i];
        while l < left {
            left -= 1;
            add(&mut state, left);
        }
        while right < r {
            right += 1;
            add(&mut state, right);
        }
        while left < l {
            remove(&mut state, left);
            left += 1;
        }
        while r < right {
            remove(&mut state, right);
            right -= 1;
        }
        ans[i] = state.count == 0;
    }

    for draw in ans {
        if draw {
            println!("YES");
        } else {
            println!("NO");
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
