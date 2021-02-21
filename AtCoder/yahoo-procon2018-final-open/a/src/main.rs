fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let size = 1_000_00 + 1;
    // dp[x]: x の倍数の個数
    let mut dp = vec![0; size];
    for &x in &a {
        dp[x] += 1;
    }
    for i in 1..size {
        for j in ((i * 2)..size).step_by(i) {
            dp[i] += dp[j];
        }
    }
    for x in 1..=m {
        let mut x = x;
        let mut factors = vec![];
        for y in 2..=x {
            if y * y > x {
                break;
            }
            if x % y == 0 {
                factors.push(y);
                while x % y == 0 {
                    x /= y;
                }
            }
        }
        if x > 1 {
            factors.push(x);
        }
        let mut ans = 0;
        for bits in 0..(1 << factors.len()) {
            let choose: Vec<usize> = (0..factors.len()).filter(|&i| bits >> i & 1 == 1).collect();
            let y = choose.iter().map(|&i| factors[i]).product::<usize>();
            if choose.len() % 2 == 1 {
                ans -= dp[y];
            } else {
                ans += dp[y];
            }
        }
        println!("{}", ans);
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
