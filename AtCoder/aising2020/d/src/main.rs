fn f(x: u32) -> u32 {
    let mut x = x;
    let mut res = 0;
    while x.count_ones() >= 1 {
        x = x % x.count_ones();
        res += 1;
    }
    res
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let x: Vec<char> = rd.get_chars();

    let one = x.iter().filter(|&&c| c == '1').count() as u32;
    let mut p2a = vec![1u32; n]; // 2^i % (one+1)
    let mut p2b = vec![1u32; n]; // 2^i % (one-1) ??
    for i in 1..n {
        p2a[i] = p2a[i - 1] * 2 % (one + 1);
        p2b[i] = (p2b[i - 1] * 2)
            .checked_rem(one.saturating_sub(1))
            .unwrap_or(0);
    }
    let mut xa = 0; // x % (i+1)
    let mut xb = 0; // x % (i-1) ??
    for i in 0..n {
        let j = n - i - 1;
        if x[i] == '1' {
            xa += p2a[j];
            xa %= one + 1;
            if one >= 2 {
                xb += p2b[j];
                xb %= one - 1;
            }
        }
    }
    for i in 0..n {
        let j = n - i - 1;
        let ans = match x[i] {
            '0' => {
                let y = (xa + p2a[j]) % (one + 1);
                1 + f(y)
            }
            '1' => {
                assert!(one >= 1);
                if one == 1 {
                    0
                } else {
                    assert!(one >= 2);
                    let y = ((one - 1) + xb - p2b[j]) % (one - 1);
                    1 + f(y)
                }
            }
            _ => unreachable!(),
        };
        println!("{}", ans);
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
