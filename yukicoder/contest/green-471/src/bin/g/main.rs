use std::{io, collections::HashSet};
use scanner::Scanner;
use union_find::UnionFind;

fn main() {
    let mut scanner = Scanner::from(io::stdin().lock());
    let (n, m) = scan!((usize, usize), <~ scanner);
    let c = scan!([usize; n], <~ scanner);
    let edges = scan!([(usize, usize); m], <~ scanner);

    let c = c.into_iter().map(|c| c - 1).collect::<Vec<_>>();
    let edges = edges.into_iter().map(|(u, v)| (u - 1, v - 1)).collect::<Vec<_>>();

    let mut sets = vec![HashSet::new(); n];
    for i in 0..n {
        sets[c[i]].insert(i);
    }
    let mut uf = UnionFind::new(n);
    for (u, v) in edges {
        if sets[c[u]].contains(&v) {
            uf.unite(u, v);
        }
    }
    let mut ans = 0;
    for set in sets {
        if set.is_empty() {
            continue;
        }
        let mut roots = Vec::new();
        for v in set {
            roots.push(uf.find(v));
        }
        roots.sort();
        roots.dedup();
        assert!(roots.len() >= 1);
        ans += roots.len() - 1;
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
