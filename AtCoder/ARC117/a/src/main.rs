use std::ops::Neg;

fn solve(a: usize, b: usize) -> Vec<i64> {
    assert!(a < b);
    let a = a as i64;
    let b = b as i64;
    let neg_sum = b * (b + 1) / 2;
    let pos_sum = a * (a + 1) / 2;
    let diff = neg_sum - pos_sum;
    assert!(diff >= b);
    let mut ans = Vec::new();
    for x in 1..=b {
        ans.push(-x);
    }
    for x in 1..a {
        ans.push(x);
    }
    ans.push(a + diff);
    ans.sort();
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: usize = rd.get();
    let b: usize = rd.get();

    let ans = if a < b {
        solve(a, b)
    } else if a > b {
        let ans = solve(b, a);
        ans.into_iter().map(|x| x.neg()).collect()
    } else {
        let mut ans = Vec::new();
        for x in 1..=(a as i64) {
            ans.push(x);
            ans.push(-x);
        }
        ans
    };
    assert_eq!(ans.len(), a + b);
    let positive = ans.iter().filter(|&&x| x > 0).count();
    let negative = ans.iter().filter(|&&x| x < 0).count();
    assert_eq!(positive, a);
    assert_eq!(negative, b);
    let mut unique: Vec<i64> = ans.iter().copied().collect();
    unique.sort();
    unique.dedup();
    assert_eq!(ans.len(), unique.len());
    let between = ans
        .iter()
        .all(|&x| -1_000_000_000 <= x && x <= 1_000_000_000);
    assert!(between);
    let mut ans = ans;
    ans.sort();
    let ans: Vec<String> = ans.into_iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(" "));
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
