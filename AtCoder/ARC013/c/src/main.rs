fn grundy(xx: u64, yy: u64, zz: u64, pts: Vec<(u64, u64, u64)>) -> u64 {
    let xmin = pts.iter().copied().map(|(x, _, _)| x).min().unwrap();
    let xmax = pts.iter().copied().map(|(x, _, _)| x).max().unwrap();
    let ymin = pts.iter().copied().map(|(_, y, _)| y).min().unwrap();
    let ymax = pts.iter().copied().map(|(_, y, _)| y).max().unwrap();
    let zmin = pts.iter().copied().map(|(_, _, z)| z).min().unwrap();
    let zmax = pts.iter().copied().map(|(_, _, z)| z).max().unwrap();
    xmin ^ (xx - xmax - 1) ^ ymin ^ (yy - ymax - 1) ^ zmin ^ (zz - zmax - 1)
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut g = 0;
    for _ in 0..n {
        let xx: u64 = rd.get();
        let yy: u64 = rd.get();
        let zz: u64 = rd.get();
        let m: usize = rd.get();
        let pts: Vec<(u64, u64, u64)> = (0..m)
            .map(|_| {
                let x: u64 = rd.get();
                let y: u64 = rd.get();
                let z: u64 = rd.get();
                (x, y, z)
            })
            .collect();
        g ^= grundy(xx, yy, zz, pts);
    }
    if g == 0 {
        println!("LOSE");
    } else {
        println!("WIN");
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
