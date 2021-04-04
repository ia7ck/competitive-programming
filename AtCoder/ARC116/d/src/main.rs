const MO: usize = 998244353;

use std::collections::HashMap;

fn solve(n: usize, m: usize, binom: &Vec<Vec<usize>>, memo: &mut HashMap<usize, usize>) -> usize {
    if m == 0 {
        return 1;
    }
    if m % 2 == 1 {
        return 0;
    }
    if let Some(&ans) = memo.get(&m) {
        return ans;
    }
    let mut ans = 0;
    for i in (0..=n).step_by(2) {
        if i <= m {
            ans += binom[n][i] * solve(n, (m - i) / 2, binom, memo);
            ans %= MO;
        }
    }
    memo.insert(m, ans);
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();

    let mut binom = vec![vec![0; n + 1]; n + 1];
    binom[1][0] = 1;
    binom[1][1] = 1;
    for i in 2..=n {
        binom[i][0] = 1;
        for j in 1..n {
            binom[i][j] = (binom[i - 1][j - 1] + binom[i - 1][j]) % MO;
        }
        binom[i][i] = 1;
    }
    let mut memo = HashMap::new();
    let ans = solve(n, m, &binom, &mut memo);
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
