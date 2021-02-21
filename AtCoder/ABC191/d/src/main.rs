fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let x: f64 = rd.get();
    let y: f64 = rd.get();
    let r: f64 = rd.get();

    let x = (x * 10000.0).round() as i64;
    let y = (y * 10000.0).round() as i64;
    let r = (r * 10000.0).round() as i64;
    let m = 10000;
    let mut ans = 0;
    for xx in -200_000..=200_000i64 {
        let xx = xx * m;
        if xx < x - r || x + r < xx {
            continue;
        }
        let s = r * r - (xx - x) * (xx - x);
        let s = sqrt(s as u64) as i64;
        let l = -s + y;
        let r = s + y;
        // [l, r] の範囲にある m の倍数
        if 0 < l {
            ans += r / m - (l - 1) / m;
        } else if r < 0 {
            let (l, r) = (-r, -l);
            ans += r / m - (l - 1) / m;
        } else {
            ans += r / m;
            ans += (-l) / m;
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn sqrt(x: u64) -> u64 {
    let sqrt_x = (x as f64).sqrt() as u64;
    (sqrt_x.saturating_sub(1)..)
        .take_while(|&y| y.checked_mul(y).map_or(false, |y2| y2 <= x))
        .last()
        .unwrap()
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
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", rest));
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
                    self.l.clear();
                    self.i = 0;
                    let num_bytes = self
                        .r
                        .read_line(&mut self.l)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = self
                        .l
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
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
