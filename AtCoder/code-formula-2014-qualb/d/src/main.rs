const M: u64 = 1_000_000_000 + 7;

use std::collections::HashMap;
fn solve(a: &[u64], carry: u64, memo: &mut HashMap<(usize, u64), u64>) -> u64 {
    if let Some(&x) = a.first() {
        if let Some(&ans) = memo.get(&(a.len(), carry)) {
            return ans;
        }
        let x = x + carry;
        let (q, r) = (x / 10, x % 10);
        let mut ans = (r + 1) * solve(&a[1..], q, memo) % M;
        if q >= 1 {
            ans += (10 - r - 1) * solve(&a[1..], q - 1, memo) % M;
            ans %= M;
        }
        memo.insert((a.len(), carry), ans);
        ans
    } else {
        (carry + 1) % M
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<u64> = rd.get_vec(n);

    let mut memo = HashMap::new();
    let ans = solve(&a, 0, &mut memo);
    println!("{}", (M + ans - 1) % M);
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
