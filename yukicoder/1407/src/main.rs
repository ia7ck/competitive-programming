const MO: u64 = 1_000_000_000 + 7;
use std::collections::HashMap;
fn solve(s: &[u64], carry: bool, memo: &mut HashMap<(usize, bool), u64>) -> u64 {
    let len = s.len();
    if len == 1 {
        let u = s[0] - carry as u64;
        return 1 + (1..=u).sum::<u64>() % MO;
    }
    if let Some(&ans) = memo.get(&(len, carry)) {
        return ans;
    }
    let t = &s[..(len - 1)];
    let ans = match (s.last(), carry) {
        (Some(0), true) => {
            let ans = (0..=9).sum::<u64>() * solve(t, true, memo);
            (ans + 1) % MO
        }
        (Some(&d), carry) => {
            let d = d - carry as u64;
            let ans = (0..=d).sum::<u64>() * solve(t, false, memo)
                + ((d + 1)..=9).sum::<u64>() * solve(t, true, memo);
            (ans + 1) % MO
        }
        _ => unreachable!(),
    };
    memo.insert((len, carry), ans);
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s: Vec<char> = rd.get_chars();
    let s: Vec<u64> = s.into_iter().map(|d| d as u64 - '0' as u64).collect();

    let mut memo = HashMap::new();
    let ans = solve(&s, false, &mut memo);
    println!("{}", (MO + ans - 1) % MO);
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
