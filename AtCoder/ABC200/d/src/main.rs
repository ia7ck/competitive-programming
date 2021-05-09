fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let m = 200;
    let find_duplicate = |vs: &Vec<Vec<usize>>| -> Option<(Vec<usize>, Vec<usize>)> {
        let mut sums = Vec::new();
        for v in vs {
            let s = v.iter().copied().map(|i| a[i]).sum::<usize>() % m;
            sums.push(s);
        }
        for j in 0..vs.len() {
            for i in 0..j {
                if sums[i] == sums[j] {
                    let b = vs[i].clone();
                    let c = vs[j].clone();
                    assert_ne!(b, c);
                    return Some((b, c));
                }
            }
        }
        None
    };

    let mut vs: Vec<Vec<usize>> = Vec::new();
    for i in 0..n {
        assert!(find_duplicate(&vs).is_none());
        let mut nxt = vs.clone();
        for mut v in vs {
            v.push(i);
            nxt.push(v);
        }
        nxt.push(vec![i]);
        vs = nxt;
        if let Some((b, c)) = find_duplicate(&vs) {
            println!("Yes");
            print!("{}", b.len());
            for b in b {
                print!(" {}", b + 1);
            }
            println!();
            print!("{}", c.len());
            for c in c {
                print!(" {}", c + 1);
            }
            println!();
            return;
        }
    }
    println!("No");
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
