use scanner::Scanner;

use std::{cmp::Reverse, collections::HashSet, io};

fn main() {
    let mut scanner = Scanner::from(io::stdin().lock());

    let t = scan!(usize, <~scanner);
    for _ in 0..t {
        let n = scan!(usize, <~scanner);
        let mut edges = Vec::new();
        for _ in 1..n {
            let (u, v) = scan!((usize, usize), <~scanner);
            edges.push((u - 1, v - 1));
        }
        solve(n, edges);
    }
}

fn solve(n: usize, edges: Vec<(usize, usize)>) {
    let mut deg = vec![0; n];
    for &(u, v) in &edges {
        deg[u] += 1;
        deg[v] += 1;
    }
    let mut ord = (0..n).collect::<Vec<_>>();
    ord.sort_by_key(|&v| Reverse(deg[v]));
    let (p, q) = (ord[0], ord[1]);
    let ans = deg[p] + deg[q] - 2;

    let mut edge_set = HashSet::new();
    for &(u, v) in &edges {
        edge_set.insert((u, v));
        edge_set.insert((v, u));
    }
    let far = {
        if deg[p] == deg[q] {
            let nodes = (0..n).filter(|&v| deg[v] == deg[p]).collect::<Vec<_>>();
            if nodes.len() >= 500 {
                // 500 * 500 > 2e5
                true
            } else {
                let mut found = false;
                'search: for (i, &u) in nodes.iter().enumerate() {
                    for &v in &nodes[..i] {
                        if !edge_set.contains(&(u, v)) {
                            found = true;
                            break 'search;
                        }
                    }
                }
                found
            }
        } else {
            // deg[p] > deg[q]
            (0..n)
                .filter(|&v| deg[v] == deg[q])
                .any(|v| !edge_set.contains(&(v, p)))
        }
    };
    if far {
        println!("{}", ans + 1);
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
