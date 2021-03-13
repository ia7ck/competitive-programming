use std::cmp::Reverse;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let mut a = a;
    a.sort();
    a.reverse();
    use std::collections::BinaryHeap;
    let mut q = BinaryHeap::new();
    while q.len() < k {
        if let Some(&x) = a.last() {
            if x == 0 {
                a.pop();
                q.push(Reverse(x));
            } else {
                break;
            }
        } else {
            break;
        }
    }
    let mut ans = 0;
    while let Some(Reverse(x)) = q.pop() {
        if let Some(&y) = a.last() {
            if x + 1 == y {
                a.pop();
                q.push(Reverse(y));
            } else if x + 1 > y {
                a.pop();
                q.push(Reverse(x)) // modosu
            } else {
                ans += x + 1;
            }
        } else {
            ans += x + 1;
        }
        // println!("a={:?} q={:?}", a, q);
    }
    println!("{}", ans);
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