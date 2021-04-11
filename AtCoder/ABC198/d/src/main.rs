fn f(s: &[char], alphs: &[char], digits: &[u64]) -> u64 {
    let mut res = 0;
    for c in s {
        res *= 10;
        let i = alphs.iter().position(|a| a == c).unwrap();
        res += digits[i];
    }
    res
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s1: Vec<char> = rd.get_chars();
    let s2: Vec<char> = rd.get_chars();
    let s3: Vec<char> = rd.get_chars();

    let mut alphs = Vec::new();
    for &c in &s1 {
        alphs.push(c);
    }
    for &c in &s2 {
        alphs.push(c);
    }
    for &c in &s3 {
        alphs.push(c);
    }
    alphs.sort();
    alphs.dedup();
    if alphs.len() >= 11 {
        println!("UNSOLVABLE");
        return;
    }
    let mut digits: Vec<u64> = (0..=9).collect();
    loop {
        let n1 = f(&s1, &alphs, &digits);
        let n2 = f(&s2, &alphs, &digits);
        let n3 = f(&s3, &alphs, &digits);
        // ['d', 'e', 'm', 'n', 'o', 'r', 's', 'y']
        // 7, 5, 1, 6, 0, 8, 9, 2
        if n1 > 0
            && n2 > 0
            && n3 > 0
            && n1 + n2 == n3
            && n1.to_string().len() == s1.len()
            && n2.to_string().len() == s2.len()
            && n3.to_string().len() == s3.len()
        {
            println!("{}", n1);
            println!("{}", n2);
            println!("{}", n3);
            return;
        }
        if !digits.next_permutation() {
            break;
        }
    }
    println!("UNSOLVABLE");
}

pub trait NextPermutation {
    fn next_permutation(&mut self) -> bool;
}

impl<T: Ord> NextPermutation for [T] {
    fn next_permutation(&mut self) -> bool {
        if self.len() <= 1 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while self[i - 1] >= self[j] {
            j -= 1;
        }
        self.swap(i - 1, j);
        self[i..].reverse();
        true
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
