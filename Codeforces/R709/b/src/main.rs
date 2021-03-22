fn solve(n: usize, a: Vec<i64>) {
    if n == 1 {
        println!("0");
        return;
    }
    let diffs: Vec<i64> = a.windows(2).map(|w| w[1] - w[0]).collect();
    let posi: Vec<i64> = diffs.iter().copied().filter(|&x| x >= 0).collect();
    let nega: Vec<i64> = diffs.iter().copied().filter(|&x| x < 0).collect();
    if posi.len() >= 1 {
        let x = posi[0];
        if posi.iter().any(|&y| y != x) {
            println!("-1");
            return;
        }
    }
    if nega.len() >= 1 {
        let x = nega[0];
        if nega.iter().any(|&y| y != x) {
            println!("-1");
            return;
        }
    }
    if posi.is_empty() || nega.is_empty() {
        println!("0");
        return;
    }
    let m = posi[0] + -1 * nega[0];
    let c = posi[0] % m;
    let mut b = vec![0; n];
    b[0] = a[0] % m;
    for i in 1..n {
        b[i] = (b[i - 1] + c) % m;
    }
    if a == b {
        println!("{} {}", m, c);
    } else {
        println!("-1");
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let a: Vec<i64> = rd.get_vec(n);
        solve(n, a);
    }
}

// 3
// 5 1 4
// c=3
// m=7

// 3
// 6 5 4
// c=6
// m=7

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
