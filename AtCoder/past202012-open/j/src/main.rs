fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s: Vec<char> = rd.get::<String>().chars().collect();
    let x: usize = rd.get();

    let inf = 1_000_000_000_000_000 + 1;

    let s: Vec<char> = vec!['1'].into_iter().chain(s.into_iter()).collect();
    let n = s.len();
    let mut len = vec![0; n];
    for i in 1..n {
        if s[i].is_alphabetic() {
            len[i] = len[i - 1] + 1;
        } else {
            let d = s[i] as usize - '0' as usize;
            len[i] = len[i - 1] * (d + 1);
        }
        len[i] = len[i].min(inf);
    }
    assert!(len[n - 1] >= x);
    let mut x = x;
    // eprintln!("{:?}", len);
    for i in (0..(n - 1)).rev() {
        // eprintln!("i={} x={}", i, x);
        if len[i] < x {
            if len[i] + 1 == len[i + 1] {
                assert_eq!(len[i + 1], x);
                assert!(s[i + 1].is_alphabetic());
                println!("{}", s[i + 1]);
                return;
            } else {
                x = (x - 1) % len[i] + 1;
            }
        }
    }
    unreachable!();
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
