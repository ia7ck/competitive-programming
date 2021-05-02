fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let abcde: Vec<(u32, u32, u32, u32, u32)> = (0..n)
        .map(|_| {
            let a: u32 = rd.get();
            let b: u32 = rd.get();
            let c: u32 = rd.get();
            let d: u32 = rd.get();
            let e: u32 = rd.get();
            (a, b, c, d, e)
        })
        .collect();
    let enough = |lb| {
        let mut abcde: Vec<(bool, bool, bool, bool, bool)> = abcde
            .iter()
            .copied()
            .map(|(a, b, c, d, e)| (a >= lb, b >= lb, c >= lb, d >= lb, e >= lb))
            .collect();
        abcde.sort();
        abcde.dedup();
        for &(ai, bi, ci, di, ei) in &abcde {
            for &(aj, bj, cj, dj, ej) in &abcde {
                for &(ak, bk, ck, dk, ek) in &abcde {
                    if (ai || aj || ak)
                        && (bi || bj || bk)
                        && (ci || cj || ck)
                        && (di || dj || dk)
                        && (ei || ej || ek)
                    {
                        return true;
                    }
                }
            }
        }
        false
    };
    let mut ok = 1;
    let mut ng = 1_000_000_000 + 1;
    while ng - ok > 1 {
        let md = (ok + ng) / 2;
        if enough(md) {
            ok = md;
        } else {
            ng = md;
        }
    }
    println!("{}", ok);
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
