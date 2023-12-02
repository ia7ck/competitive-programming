use std::{io, collections::VecDeque};
use scanner::Scanner;

const INF: u64 = u64::MAX / 3;

fn bfs(s: usize, g: &Vec<Vec<usize>>) -> Vec<u64> {
    let mut dist = vec![INF; g.len()];
    let mut que = VecDeque::new();
    dist[s] = 0;
    que.push_back(s);
    while let Some(v) = que.pop_front() {
        for &x in &g[v] {
            if dist[x] == INF {
                dist[x] = dist[v] + 1;
                que.push_back(x);
            }
        }
    }
    dist
}

fn main() {
    let mut scanner = Scanner::from(io::stdin().lock());
    let (n, m) = scan!((usize, usize), <~ scanner);
    let edges = scan!([(usize, usize); m], <~ scanner);

    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u - 1].push(v - 1);
    }

    let (x, y) = (n - 2, n - 1);
    let d_0 = bfs(0, &g);
    let d_x = bfs(x, &g);
    let d_y = bfs(y, &g);

    let mut ans = INF;
    // 0 -> x -> y -> 0
    ans = ans.min(d_0[x] + d_x[y] + d_y[0]);
    // 0 -> y -> x -> 0
    ans = ans.min(d_0[y] + d_y[x] + d_x[0]);

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
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
