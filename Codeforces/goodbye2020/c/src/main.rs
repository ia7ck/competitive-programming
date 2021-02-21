fn select(ng: &[char]) -> char {
    for d in 0..26 {
        let ch = ('a' as u8 + d) as char;
        if !ng.contains(&ch) {
            return ch;
        }
    }
    unreachable!();
}

fn solve(s: Vec<char>) {
    let mut s = s;
    let n = s.len();
    let mut ans = 0;
    for i in 1..n {
        let mut t = vec![];
        t.push(s[i - 1]);
        if i >= 2 {
            t.push(s[i - 2]);
        }
        if t.contains(&s[i]) {
            ans += 1;
            if i + 1 < n {
                t.push(s[i + 1]);
            }
            if i + 2 < n {
                t.push(s[i + 2]);
            }
            let ch = select(&t);
            s[i] = ch;
        }
    }
    // eprintln!("{:?}", s);
    println!("{}", ans);
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let s: Vec<char> = rd.get::<String>().chars().collect();
        solve(s);
    }
}

pub struct ProconReader<R> {
    r: R,
    line: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            line: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.line.len());
        assert_ne!(&self.line[self.i..=self.i], " ");
        let line = &self.line[self.i..];
        let end = line.find(' ').unwrap_or_else(|| line.len());
        let s = &line[..end];
        self.i += end;
        s.parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", self.line))
    }
    fn skip_blanks(&mut self) {
        loop {
            let start = self.line[self.i..].find(|ch| ch != ' ');
            match start {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    self.line.clear();
                    self.i = 0;
                    let num_bytes = self.r.read_line(&mut self.line).unwrap();
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.line = self.line.trim_end_matches(&['\r', '\n'][..]).to_string();
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
}
