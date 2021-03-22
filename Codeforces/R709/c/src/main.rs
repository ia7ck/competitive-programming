fn solve(n: usize, m: usize, a: Vec<Vec<usize>>) {
    let mut freq = vec![0; n + 1];
    for r in &a {
        for &x in r {
            freq[x] += 1;
        }
    }
    let max = freq.iter().copied().max().unwrap();
    let lim = (m + 1) / 2;
    let mut ans = Vec::new();
    if max > lim {
        ans = vec![0; m];
        let mut a: Vec<(usize, Vec<usize>)> = a.into_iter().enumerate().collect();
        a.sort_by_key(|(_, r)| r.len());
        let tgt = freq.iter().position(|&x| x == max).unwrap();
        let mut choose = 0;
        for (i, a) in a {
            if choose < lim && a.contains(&tgt) {
                choose += 1;
                ans[i] = tgt;
            } else if let Some(oth) = a.iter().copied().find(|&x| x != tgt) {
                ans[i] = oth;
            }
        }
    } else {
        for a in a {
            ans.push(a[0]);
        }
    }
    if ans.iter().any(|&x| x == 0) {
        println!("NO");
    } else {
        println!("YES");
        let ans: Vec<String> = ans.into_iter().map(|x| x.to_string()).collect();
        println!("{}", ans.join(" "));
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let m: usize = rd.get();
        let a: Vec<Vec<usize>> = (0..m)
            .map(|_| {
                let k: usize = rd.get();
                rd.get_vec(k)
            })
            .collect();
        solve(n, m, a);
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
