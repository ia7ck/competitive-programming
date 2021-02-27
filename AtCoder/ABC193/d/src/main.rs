fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let k: usize = rd.get();
    let s: Vec<char> = rd.get_chars();
    let t: Vec<char> = rd.get_chars();

    let s: Vec<usize> = s[..4].iter().map(|&c| c as usize - '0' as usize).collect();
    let t: Vec<usize> = t[..4].iter().map(|&c| c as usize - '0' as usize).collect();
    let mut freq = vec![k; 10];
    // freq[0] = 0; //
    for &x in &s {
        freq[x] -= 1;
    }
    for &x in &t {
        freq[x] -= 1;
    }
    fn calc(c: &[usize]) -> usize {
        assert_eq!(c.len(), 10);
        (1..=9).map(|i| i * 10usize.pow(c[i] as u32)).sum::<usize>()
    }
    let judge = |s5: usize, t5: usize| -> bool {
        let mut cs = vec![0; 10];
        let mut ct = vec![0; 10];
        for &x in &s {
            cs[x] += 1;
        }
        for &x in &t {
            ct[x] += 1;
        }
        cs[s5] += 1;
        ct[t5] += 1;
        calc(&cs) > calc(&ct)
    };
    let mut ans = 0;
    for a in 1..=9 {
        if freq[a] == 0 {
            continue;
        }
        let choose_a = freq[a];
        freq[a] -= 1;
        for b in 1..=9 {
            if freq[b] == 0 {
                continue;
            }
            let choose_b = freq[b];
            freq[b] -= 1;
            if judge(a, b) {
                ans += choose_a * choose_b;
            }
            freq[b] += 1;
        }
        freq[a] += 1;
    }
    let denom = (k * 9 - 8) * (k * 9 - 9);
    let ans = ans as f64 / denom as f64;
    println!("{}", ans);
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
