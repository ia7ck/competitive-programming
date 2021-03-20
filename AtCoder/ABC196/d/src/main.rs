fn solve(h: usize, w: usize, a: usize, b: usize, i: usize, c: &mut Vec<Vec<bool>>) -> u32 {
    if i == h * w {
        return 1;
    }
    let (y, x) = (i / w, i % w);
    let mut ans = 0;
    if c[y][x] {
        ans += solve(h, w, a, b, i + 1, c);
    }
    if !c[y][x] && b >= 1 {
        c[y][x] = true;
        ans += solve(h, w, a, b - 1, i + 1, c);
        c[y][x] = false;
    }
    if !c[y][x] && x + 1 < w && !c[y][x + 1] && a >= 1 {
        c[y][x] = true;
        c[y][x + 1] = true;
        ans += solve(h, w, a - 1, b, i + 1, c);
        c[y][x] = false;
        c[y][x + 1] = false;
    }
    if !c[y][x] && y + 1 < h && !c[y + 1][x] && a >= 1 {
        c[y][x] = true;
        c[y + 1][x] = true;
        ans += solve(h, w, a - 1, b, i + 1, c);
        c[y][x] = false;
        c[y + 1][x] = false;
    }
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let a: usize = rd.get();
    let b: usize = rd.get();

    let mut c = vec![vec![false; w]; h];
    let ans = solve(h, w, a, b, 0, &mut c);
    println!("{}", ans);
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
