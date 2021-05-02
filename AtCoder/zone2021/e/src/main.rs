fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let a: Vec<Vec<u64>> = (0..h).map(|_| rd.get_vec(w - 1)).collect();
    let b: Vec<Vec<u64>> = (0..(h - 1)).map(|_| rd.get_vec(w)).collect();

    let inf = std::u64::MAX / 2;
    let mut d = vec![vec![vec![inf; 2]; w]; h];
    d[0][0][0] = 0;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), 0, 0, false));
    while let Some((Reverse(cur), i, j, up)) = q.pop() {
        if cur > d[i][j][up as usize] {
            continue;
        }
        if j + 1 < w {
            let nxt = cur + a[i][j];
            if nxt < d[i][j + 1][0] {
                d[i][j + 1][0] = nxt;
                q.push((Reverse(nxt), i, j + 1, false));
            }
        }
        if j >= 1 {
            let nxt = cur + a[i][j - 1];
            if nxt < d[i][j - 1][0] {
                d[i][j - 1][0] = nxt;
                q.push((Reverse(nxt), i, j - 1, false));
            }
        }
        if i + 1 < h {
            let nxt = cur + b[i][j];
            if nxt < d[i + 1][j][0] {
                d[i + 1][j][0] = nxt;
                q.push((Reverse(nxt), i + 1, j, false));
            }
        }
        if i >= 1 {
            let nxt = cur + if up { 1 } else { 2 };
            if nxt < d[i - 1][j][1] {
                d[i - 1][j][1] = nxt;
                q.push((Reverse(nxt), i - 1, j, true));
            }
        }
    }
    let ans = d[h - 1][w - 1][0];
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
