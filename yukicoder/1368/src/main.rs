fn is_kado(a: &[u64]) -> bool {
    assert_eq!(a.len(), 3);
    if a[0] == a[1] || a[1] == a[2] || a[2] == a[0] {
        return false;
    }
    return (a[0] < a[1] && a[1] > a[2]) || (a[0] > a[1] && a[1] < a[2]);
}

fn f(n: usize, a: Vec<u64>) -> u64 {
    let mut dp = vec![0; n];
    dp[0] = 0;
    dp[1] = 0;
    if is_kado(&a[0..=2]) {
        dp[2] = a[0];
    }
    for i in 3..n {
        dp[i] = dp[i - 1];
        if is_kado(&a[(i - 2)..=i]) {
            dp[i] = dp[i].max(dp[i - 3] + a[i - 2]);
        }
    }
    dp[n - 1]
}

fn solve(n: usize, a: Vec<u64>) {
    let mut ans = 0;
    for &i in &[1, 2] {
        let mut b = a.clone();
        b.rotate_left(i);
        ans = ans.max(f(n, b));
        let mut c = a.clone();
        c.rotate_right(i);
        ans = ans.max(f(n, c));
    }
    ans = ans.max(f(n, a));
    println!("{}", ans);
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let a: Vec<u64> = rd.get_vec(n);
        solve(n, a);
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
