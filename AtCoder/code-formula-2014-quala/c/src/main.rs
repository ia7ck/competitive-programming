fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();
    let a: Vec<Vec<usize>> = (0..n).map(|_| rd.get_vec(k)).collect();

    let mut cap = k - 1;
    let mut pos = vec![0; n];
    let mut seen = vec![false; 999999 + 1];
    for i in 0..n {
        let mut ans = Vec::new();
        loop {
            let mut ex = 0;
            for ii in 0..=i {
                while pos[ii] < k && ii + pos[ii] * n <= cap {
                    let x = a[ii][pos[ii]];
                    if seen[x] {
                        ex += 1;
                    } else {
                        seen[x] = true;
                        ans.push(x);
                    }
                    pos[ii] += 1;
                }
            }
            if ex == 0 {
                break;
            }
            cap += ex;
        }
        ans.sort();
        println!("{}", ans.link(' '));
    }
}

trait Link {
    fn link(&self, separator: char) -> String;
}

impl<T> Link for [T]
where
    T: std::fmt::Display,
{
    fn link(&self, separator: char) -> String {
        let mut res = String::new();
        let mut it = self.iter();
        if let Some(head) = it.next() {
            res.push_str(&format!("{}", head));
            for x in it {
                res.push_str(&format!("{}{}", separator, x));
            }
        }
        res
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
