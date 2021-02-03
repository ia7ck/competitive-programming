fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let xy: Vec<(i64, i64)> = (0..n)
        .map(|_| {
            let x: i64 = rd.get();
            let y: i64 = rd.get();
            (x, y)
        })
        .collect();
    let m: usize = rd.get();
    let mut a = Vec::new();
    a.push(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
    for o in 0..m {
        let tp: usize = rd.get();
        let b = match tp {
            1 => vec![vec![0, 1, 0], vec![-1, 0, 0], vec![0, 0, 1]],
            2 => vec![vec![0, -1, 0], vec![1, 0, 0], vec![0, 0, 1]],
            3 => {
                let p: i64 = rd.get();
                vec![vec![-1, 0, p * 2], vec![0, 1, 0], vec![0, 0, 1]]
            }
            4 => {
                let p: i64 = rd.get();
                vec![vec![1, 0, 0], vec![0, -1, p * 2], vec![0, 0, 1]]
            }
            _ => unreachable!(),
        };
        let mut c = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    c[i][j] += b[i][k] * a[o][k][j];
                }
            }
        }
        a.push(c);
    }
    let q: usize = rd.get();
    for _ in 0..q {
        let i: usize = rd.get();
        let j: usize = rd.get();
        let a = &a[i];
        let (x, y) = xy[j - 1];
        println!(
            "{} {}",
            a[0][0] * x + a[0][1] * y + a[0][2],
            a[1][0] * x + a[1][1] * y + a[1][2]
        );
    }
}

pub struct ProconReader<R> {
    r: R,
    line: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            line: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.line.len());
        assert_ne!(&self.line[self.i..=self.i], " ");
        let line = &self.line[self.i..];
        let end = line.find(' ').unwrap_or_else(|| line.len());
        let s = &line[..end];
        self.i += end;
        s.parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", self.line))
    }
    fn skip_blanks(&mut self) {
        loop {
            let start = self.line[self.i..].find(|ch| ch != ' ');
            match start {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    self.line.clear();
                    self.i = 0;
                    let num_bytes = self.r.read_line(&mut self.line).unwrap();
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.line = self.line.trim_end_matches(&['\r', '\n'][..]).to_string();
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
}
