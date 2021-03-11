fn dfs(i: usize, j: usize, m: usize, n: usize, a: &mut Vec<Vec<usize>>) -> u32 {
    if i >= m {
        return 1;
    }
    let mut res = 0;
    for &x in &[1, 2, 3] {
        let up: Vec<usize> = ((i.saturating_sub(x))..i).map(|i| a[i][j]).collect();
        let left: Vec<usize> = ((j.saturating_sub(x))..j).map(|j| a[i][j]).collect();
        if up.iter().all(|&y| y != x) && left.iter().all(|&y| y != x) {
            a[i][j] = x;
            let (ni, nj) = if j + 1 < n { (i, j + 1) } else { (i + 1, 0) };
            res += dfs(ni, nj, m, n, a);
            a[i][j] = 0;
        }
    }
    res
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let m: usize = rd.get();
    let n: usize = rd.get();

    let m = if m >= 20 { 20 + m % 4 } else { m };
    let n = if n >= 20 { 20 + n % 4 } else { n };
    let mut a = vec![vec![0; n]; m];
    let ans = dfs(0, 0, m, n, &mut a);
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
