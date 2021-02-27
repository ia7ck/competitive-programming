use std::collections::HashSet;
fn solve(s: &str, adj: &[HashSet<usize>], rev_adj: &[HashSet<usize>]) -> HashSet<usize> {
    if s.starts_with("group") {
        // group123w
        // group123
        let g = s.trim_start_matches("group").trim_end_matches('w');
        let g = g.parse::<usize>().unwrap() - 1;
        if s.ends_with('w') {
            rev_adj[g].clone()
        } else {
            let mut set: HashSet<_> = (0..adj.len()).collect();
            for i in 0..adj.len() {
                if adj[i].contains(&g) {
                    set.remove(&i);
                }
            }
            set
        }
    } else {
        assert!(s.starts_with('"'));
        if s.ends_with('"') {
            // "..."
            let gs = solve(&s[1..(s.len() - 1)], adj, rev_adj);
            let mut set: HashSet<_> = (0..adj.len()).collect();
            for i in 0..adj.len() {
                if adj[i].is_superset(&gs) {
                    set.remove(&i);
                }
            }
            set
        } else {
            // "..."ww
            assert!(s.ends_with("ww"));
            let gs = solve(&s[1..(s.len() - 3)], adj, rev_adj);
            let mut hate_gs = HashSet::new();
            for g in gs {
                hate_gs.extend(&rev_adj[g]);
            }
            hate_gs
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let ab: Vec<(usize, usize)> = (0..m)
        .map(|_| {
            let a: usize = rd.get();
            let b: usize = rd.get();
            (a - 1, b - 1)
        })
        .collect();
    let s: String = rd.get();

    let mut adj = vec![HashSet::new(); n];
    let mut rev_adj = vec![HashSet::new(); n];
    for (a, b) in ab {
        adj[a].insert(b);
        rev_adj[b].insert(a);
    }
    let candidates = solve(&s, &adj, &rev_adj);
    println!("{}", candidates.len());
}

pub struct ProconReader<R> {
    r: R,
    l: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            l: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.l.len()); // remain some character
        assert_ne!(&self.l[self.i..=self.i], " ");
        let rest = &self.l[self.i..];
        let len = rest.find(' ').unwrap_or_else(|| rest.len());
        // parse self.l[self.i..(self.i + len)]
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|e| panic!("{:?}, attempt to read `{}`", e, rest));
        self.i += len;
        val
    }
    fn skip_blanks(&mut self) {
        loop {
            match self.l[self.i..].find(|ch| ch != ' ') {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    let mut buf = String::new();
                    let num_bytes = self
                        .r
                        .read_line(&mut buf)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = buf
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
                    self.i = 0;
                }
            }
        }
    }
    pub fn get_vec<T>(&mut self, n: usize) -> Vec<T>
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.get()).collect()
    }
    pub fn get_chars(&mut self) -> Vec<char> {
        self.get::<String>().chars().collect()
    }
}
