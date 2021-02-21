fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let mut dp = vec![0; n];
    dp[n - 1] = 1;
    for i in (0..(n - 1)).rev() {
        if a[i] < a[i + 1] {
            dp[i] = dp[i + 1] + 1;
        } else {
            dp[i] = 1;
        }
    }

    use std::collections::BTreeSet;
    let mut set = BTreeSet::new();
    for i in 0..k {
        set.insert(a[i]);
    }
    let mut ans = 1;
    let mut seen = dp[0] >= k;
    for i in 0..n {
        // [i, i+k) -> [i+1, i+1+k)
        if i + 1 + k > n {
            continue;
        }
        set.insert(a[i + 1 + k - 1]);
        let &min = set.iter().next().unwrap();
        let &max = set.iter().last().unwrap();
        if (a[i] == min && a[i + 1 + k - 1] == max) || (seen && dp[i + 1] >= k) {
            //
        } else {
            ans += 1;
        }
        set.remove(&a[i]);
        seen |= dp[i + 1] >= k;
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
