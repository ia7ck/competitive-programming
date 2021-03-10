fn solve(_: usize, xy: Vec<(i32, i32)>) {
    let mut y: Vec<i32> = xy
        .iter()
        .copied()
        .filter(|&(x, _)| x == 0)
        .map(|(_, y)| y.abs())
        // .map(|(_, y)| y)
        .collect();
    let mut x: Vec<i32> = xy
        .iter()
        .copied()
        .filter(|&(_, y)| y == 0)
        .map(|(x, _)| x.abs())
        // .map(|(x, _)| x)
        .collect();
    assert_eq!(x.len(), y.len());
    y.sort();
    x.sort();
    // x.reverse();
    let mut ans = 0.0;
    for (&x, &y) in x.iter().zip(y.iter()) {
        let x = x as f64;
        let y = y as f64;
        ans += x.hypot(y);
    }
    println!("{}", ans);
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let xy: Vec<(i32, i32)> = (0..(n * 2))
            .map(|_| {
                let x: i32 = rd.get();
                let y: i32 = rd.get();
                (x, y)
            })
            .collect();
        solve(n, xy);
    }
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
