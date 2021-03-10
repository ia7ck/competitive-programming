fn mex(a: &[u32]) -> u32 {
    use std::collections::HashSet;
    let a: HashSet<u32> = a.iter().copied().collect();
    for x in 0.. {
        if !a.contains(&x) {
            return x;
        }
    }
    unreachable!()
}

fn solve(n: usize, k: usize, a: Vec<u32>) {
    let me = mex(&a);
    let ma = a.iter().copied().max().unwrap();
    if me > ma {
        println!("{}", n + k);
        return;
    }
    let k = k.min(1);
    use std::collections::HashSet;
    let mut a: HashSet<u32> = a.iter().copied().collect();
    for _ in 0..k {
        fn div_ceil(x: u32, y: u32) -> u32 {
            let res = x / y;
            if x % y == 0 {
                res
            } else {
                res + 1
            }
        }
        a.insert(div_ceil(me + ma, 2));
    }
    let ans = a.len();
    println!("{}", ans);
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let k: usize = rd.get();
        let a: Vec<u32> = rd.get_vec(n);
        solve(n, k, a);
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
