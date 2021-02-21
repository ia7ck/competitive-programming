fn mpow(a: i64, x: i64, m: i64) -> i64 {
    match x {
        0 => 1,
        1 => a % m,
        x if x % 2 == 0 => mpow(a * a % m, x / 2, m),
        _ => a * mpow(a, x - 1, m) % m,
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let mo = 1_000_000_000 + 7;

    let size = 1_000_00 + 1;
    // dp[x]: x の倍数の個数
    let mut dp = vec![0; size];
    for &x in &a {
        dp[x] += 1;
    }
    // ep[x]: gcd が x の倍数になる通り数
    let mut ep = vec![0; size];
    for i in 1..size {
        for j in ((i * 2)..size).step_by(i) {
            dp[i] += dp[j];
        }
        ep[i] = (mpow(2, dp[i], mo) - 1).rem_euclid(mo);
    }

    // ans[x]: gcd が x になる通り数
    // ans[x] = ep[x] - ans[x * 2] - ans[x * 3] - ...
    let mut ans = vec![0; size];
    for i in (1..size).rev() {
        ans[i] = ep[i];
        for j in ((i * 2)..size).step_by(i) {
            ans[i] = (ans[i] - ans[j]).rem_euclid(mo);
        }
    }
    println!("{}", ans[1]);
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
