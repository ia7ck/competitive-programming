fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let r1: usize = rd.get();
    let c1: usize = rd.get();
    let r2: usize = rd.get();
    let c2: usize = rd.get();

    let mo = 1_000_000_000 + 7;
    let binom = Binom::new(r2 + c2 + 2, mo);
    let g = |r: usize, c: usize| {
        (0..=r)
            .map(|i| binom.get(i + c + 1, c))
            .fold(0, |acc, x| (acc + x) % mo)
    };
    let ans = g(r2, c2) - g(r1 - 1, c2) - g(r2, c1 - 1) + g(r1 - 1, c1 - 1);
    println!("{}", ans.rem_euclid(mo));
}

use std::fmt::Debug;
use std::str::FromStr;

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
        T: FromStr,
        <T as FromStr>::Err: Debug,
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
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        (0..n).map(|_| self.get()).collect()
    }
}

pub trait BinomialCoefficient {
    type Output;
    fn get(&self, n: usize, k: usize) -> Self::Output;
}

pub struct Binom {
    size: usize,
    mo: i64,
    fac: Vec<i64>,
    inv_fac: Vec<i64>,
}

impl Binom {
    pub fn new(size: usize, mo: i64) -> Self {
        let mut fac = vec![0; size];
        let mut inv = vec![0; size];
        let mut inv_fac = vec![0; size];
        fac[0] = 1;
        fac[1] = 1;
        inv[1] = 1;
        inv_fac[0] = 1;
        inv_fac[1] = 1;
        for i in 2..size {
            fac[i] = fac[i - 1] * (i as i64) % mo;
            inv[i] = (-inv[(mo as usize) % i] * (mo / (i as i64))).rem_euclid(mo);
            inv_fac[i] = inv_fac[i - 1] * inv[i] % mo;
        }
        Self {
            size,
            mo,
            fac,
            inv_fac,
        }
    }
}

impl BinomialCoefficient for Binom {
    type Output = i64;
    fn get(&self, n: usize, k: usize) -> Self::Output {
        assert!(n < self.size);
        if n < k {
            return 0;
        }
        ((self.fac[n] * self.inv_fac[k]) % self.mo * self.inv_fac[n - k]) % self.mo
    }
}
