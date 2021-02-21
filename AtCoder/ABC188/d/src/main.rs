fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let cc: u64 = rd.get();
    let abc: Vec<(u32, u32, u64)> = (0..n)
        .map(|_| {
            let a: u32 = rd.get();
            let b: u32 = rd.get();
            let c: u64 = rd.get();
            (a, b, c)
        })
        .collect();

    let mut events = vec![];
    for i in 0..n {
        let (a, b, c) = abc[i];
        events.push((a, 1, c));
        events.push((b + 1, 0, c));
    }
    events.sort();
    // println!("{:?}", events);
    let mut prev = events[0].0;
    let mut s = 0;
    let mut ans = 0;
    for (t, o, c) in events {
        let d = (t - prev) as u64;
        match o {
            1 => {
                // start
                ans += d * std::cmp::min(s, cc);
                s += c;
            }
            0 => {
                // end
                ans += d * std::cmp::min(s, cc);
                s -= c;
            }
            _ => unreachable!(),
        }
        prev = t;
    }
    println!("{}", ans);
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
