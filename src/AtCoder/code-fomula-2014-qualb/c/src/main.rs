fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: Vec<char> = rd.get_chars();
    let b: Vec<char> = rd.get_chars();

    let n = a.len();
    let diffs: Vec<usize> = (0..n).filter(|&i| a[i] != b[i]).collect();
    if diffs.len() > 6 {
        println!("NO");
        return;
    }
    let mut keep = Vec::new();
    for c in b'a'..=b'z' {
        let c = c as char;
        if let Some(left) = a.iter().position(|&x| x == c) {
            let right = a.iter().rposition(|&x| x == c).unwrap();
            if left != right {
                keep.push(left);
                keep.push(right);
                break;
            }
        }
    }
    keep.extend(diffs);
    keep.sort();
    keep.dedup();
    let a: Vec<char> = keep.iter().map(|&i| a[i]).collect();
    let b: Vec<char> = keep.iter().map(|&i| b[i]).collect();
    let n = a.len();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            for p in 0..n {
                for q in 0..n {
                    if p == q {
                        continue;
                    }
                    for x in 0..n {
                        for y in 0..n {
                            if x == y {
                                continue;
                            }
                            let mut a = a.clone();
                            a.swap(i, j);
                            a.swap(p, q);
                            a.swap(x, y);
                            if a == b {
                                println!("YES");
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("NO");
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
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", rest));
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
                    self.l.clear();
                    self.i = 0;
                    let num_bytes = self
                        .r
                        .read_line(&mut self.l)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = self
                        .l
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
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
