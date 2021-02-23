fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let s: Vec<char> = rd.get_chars();
    let t: Vec<char> = rd.get_chars();

    let mut a = {
        let mut i = 0;
        let mut a = Vec::new();
        for c in t {
            while s[i] != c && i < n {
                i += 1;
            }
            assert!(i < n);
            a.push(i);
            i += 1;
        }
        assert_eq!(a.len(), m);
        a
    };
    let mut left = vec![vec![None; n]; 26];
    left[s[0] as usize - 'a' as usize][0] = Some(0);
    for i in 1..n {
        for j in 0..26 {
            left[j][i] = left[j][i - 1];
        }
        left[s[i] as usize - 'a' as usize][i] = Some(i);
    }
    let mut ans = 1;
    for i in 1..m {
        ans = ans.max(a[i] - a[i - 1]);
    }
    for i in (1..m).rev() {
        let j = if i + 1 < m { a[i + 1] - 1 } else { n - 1 };
        let c = s[a[i]];
        a[i] = left[c as usize - 'a' as usize][j].unwrap();
        ans = ans.max(a[i] - a[i - 1]);
    }
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
