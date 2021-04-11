
fn solve(n: usize, a: Vec<Vec<char>>) {
    let rows: Vec<usize> = (0..n).filter(|&i| a[i].contains(&'*')).collect();
    let mut cols: Vec<usize> = Vec::new();
    for j in 0..n {
        let exist = (0..n).any(|i| a[i][j] == '*');
        if exist {
            cols.push(j);
        }
    }
    let mut points = Vec::new();
    for i in rows {
        for j in 0..n {
            if a[i][j] == '.' {
                points.push((i, j));
            }
        }
    }
    for i in 0..n {
        for &j in &cols {
            if a[i][j] == '.' {
                points.push((i, j));
            }
        }
    }

    let mut asts = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == '*' {
                asts.push((i, j));
            }
        }
    }
    for (idx, &(i, j)) in points.iter().enumerate() {
        for &(y, x) in &points[..idx] {
            let mut asts = asts.clone();
            asts.push((i, j));
            asts.push((y, x));
            asts.sort();
            let (top, left) = asts[0];
            let (bottom, right) = asts[3];
            let ok =
                top == asts[1].0 && asts[1].1 == right && bottom == asts[2].0 && asts[2].1 == left;
            if ok {
                let mut a = a;
                a[i][j] = '*';
                a[y][x] = '*';
                for row in a {
                    let row: Vec<String> = row.into_iter().map(|c| c.to_string()).collect();
                    println!("{}", row.join(""));
                }
                return;
            }
        }
    }
    unreachable!();
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let a: Vec<Vec<char>> = (0..n).map(|_| rd.get_chars()).collect();
        solve(n, a);
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
