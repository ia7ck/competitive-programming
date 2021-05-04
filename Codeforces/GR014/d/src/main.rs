fn solve(n: usize, l: usize, r: usize, a: Vec<usize>) {
    assert_eq!(n, l + r);
    let mut b = vec![0; n + 1];
    let mut c = vec![0; n + 1];
    for &x in &a[..l] {
        b[x] += 1;
    }
    for &x in &a[l..] {
        c[x] += 1;
    }
    for x in 1..=n {
        let min = b[x].min(c[x]);
        b[x] -= min;
        c[x] -= min;
    }
    let (mut b, c) = if b.iter().sum::<u32>() >= c.iter().sum::<u32>() {
        (b, c)
    } else {
        (c, b)
    };
    let diff = b.iter().sum::<u32>() - c.iter().sum::<u32>();
    assert_eq!(diff % 2, 0);
    let mut diff = diff / 2;
    let mut ans = 0;
    for x in 1..=n {
        while diff >= 1 && b[x] >= 2 {
            ans += 1;
            diff -= 1;
            b[x] -= 2;
        }
    }
    let b = b.iter().sum::<u32>();
    let c = c.iter().sum::<u32>();
    if diff == 0 {
        assert_eq!(b, c);
        ans += b;
    } else {
        assert!(b >= c);
        assert_eq!((b + c) % 2, 0);
        ans += b - (b + c) / 2;
        ans += (b + c) / 2;
    }
    println!("{}", ans);
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let l: usize = rd.get();
        let r: usize = rd.get();
        let a: Vec<usize> = rd.get_vec(n);
        solve(n, l, r, a);
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
